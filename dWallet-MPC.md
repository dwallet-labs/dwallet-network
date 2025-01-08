# Full Tx Flow

## LifeCycle of a Transaction

[See image here](https://docs.sui.io/concepts/sui-architecture/transaction-lifecycle#execution)

The following steps align with those in the preceding image.

1. The first step of the process is the creation of a transaction.
   A user with a private key creates and signs a user transaction to either mutate objects they own, or a mix of objects
   they own and shared objects.

2. Sui sends the transaction to each validator (often through a Full node).
   Validators perform a series of validity and safety checks, sign it, and return the signed transaction to the client.

3. The client then collects the responses from a set of validators that account for at least 2/3 stake on Sui (a
   supermajority) to form a transaction certificate.
   As a result, unlike consensus-based blockchains, Sui validators do not need to propagate signatures on a best-effort
   basis (gossip signatures) or aggregate certificates.
   This effort is now the responsibility of the client or gateway.

4. After assembling the certificate, the client sends it back to all validators, who check its validity and acknowledge
   its receipt to the client.
   If the transaction involves owned objects exclusively, Sui can process the transaction certificate immediately and
   execute it without waiting for the consensus engine (direct fast path).
   All certificates are forwarded to the Sui DAG-based consensus protocol (also operated by the Sui validators).
5. Consensus eventually outputs a total order of certificates; the validators check and execute those that contain
   shared objects.
6. Clients can collect a supermajority of validator responses, assemble them into an effect certificate, and use it as
   proof of the settlement of the transaction.
7. Subsequently, Sui forms checkpoints for every consensus commit, which it also uses to drive the reconfiguration
   protocol.

## DKG Flow

### Client Side

> LifeCycle part 1.

- The User initiates a dWallet DKG first round by calling the `launch_dkg_first_round()` Move function from the
  [dwallet_2pc_mpc_ecdsa_k1.move](crates/pera-framework/packages/pera-system/sources/dwallet_2pc_mpc_ecdsa_k1.move)
  module, this function is called as part of transaction submitted using TypeScript SDK or CLI, on both cases the user
  submitted this TX to a Full Node, the full node is receiving the TX using a REST API.

### Full Node

> LifeCycle part 1.

- The REST API is started by the `build_http_server()` function in `crates/pera-node/src/lib.rs`, transactions are
  executed by the `TransactionExecutionApi` which is using the `TransactiondOrchestrator`.
  `TransactionExecutionApi` is created by `JsonRpcServerBuilder` or by `RestService` if
  `config.enable_experimental_rest_api` is set to `true`.

- The old version that's using `JsonRpcServerBuilder` works as follows:

```rust
// Inside the build_http_server function:
// let json_rpc_router = {
// ..
if let Some(transaction_orchestrator) = transaction_orchestrator {
server.register_module(TransactionExecutionApi::new(
state.clone(),
transaction_orchestrator.clone(),
metrics.clone(),
)) ?;
}
// }
// ...
```

- `register_module()` calls for: `Ok(self.module.merge(module.rpc())?)`
- The `rpc()` function is part of a trait:

```Rust
pub trait PeraRpcModule
where
    Self: Sized,
{
    fn rpc(self) -> RpcModule<Self>;
    fn rpc_doc_module() -> Module;
}
```

- Implemented as such:

```Rust
impl PeraRpcModule for TransactionExecutionApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        WriteApiOpenRpc::module_doc()
    }
}
```

- And the `into_rpc()` exposes the `WriterAPI` with the function:

```Rust
pub trait WriteApi {
    /// Execute the transaction and wait for results if desired.
    /// Request types:
    /// 1. WaitForEffectsCert: wait for TransactionEffectsCert and then return to a client.
    ///     This mode is a proxy for transaction finality.
    /// 2. WaitForLocalExecution: wait for TransactionEffectsCert and make sure the node
    ///     executed the transaction locally before returning the client. The local execution
    ///     makes sure this node is aware of this transaction when a client fires subsequent queries.
    ///     However, if the node fails to execute the transaction locally in a timely manner,
    ///     a bool type in the response is set to false to indicate the case.
    /// Request_type is default to be `WaitForEffectsCert` unless options.show_events or options.show_effects is true.
    #[method(name = "executeTransactionBlock")]
    async fn execute_transaction_block() {}
```

- If `enable experimental REST API` is set to true, the `RestService` is used instead of `JsonRpcServerBuilder`:

```Rust
if let Some(transaction_orchestrator) = transaction_orchestrator {
rest_service.with_executor(transaction_orchestrator.clone())
}
router = router.merge(rest_service.into_router());
```

the `into_router()` function created the router and endpoints as such:

```Rust
    /// Convert the RestService into an axum Router.
pub fn into_router(self) -> Router {
    let metrics = self.metrics.clone();

    let mut api = openapi::Api::new(info());

    api.register_endpoints(ENDPOINTS.to_owned());

    Router::new()
        .nest("/v2/", api.to_router().with_state(self.clone()))
        .route("/v2", get(|| async { Redirect::permanent("/v2/") }))
    // etc...
}
```

- The API::new() creates a new OpenAPI object, and the `register_endpoints()` function is registering the endpoints
  defined in the `ENDPOINTS` constant.
- Then the `to_router()` function is converting the OpenAPI object to an axum Router object, and the `with_state()`
  function
  is setting the state of the router to the current `RestService` object.

```Rust
    pub fn to_router(&self) -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let mut router = OpenApiDocument::new(self.openapi()).into_router();
    for endpoint in &self.endpoints {
        let handler = endpoint.handler();
        assert_eq!(handler.method(), endpoint.method());

        // we need to replace any path parameters wrapped in braces to be prefaced by a colon
        // until axum updates matchit: https://github.com/tokio-rs/axum/pull/2645
        let path = endpoint.path().replace('{', ":").replace('}', "");

        router = router.route(&path, handler.handler);
    }

    router
}
```

- One of the endpoints is `ExecuteTransaction`, which implements the `ApiEndpoint` trait:

```Rust
impl ApiEndpoint<RestService> for ExecuteTransaction {}
```

- And calls the transaction execution:

```Rust
fn handler(&self) -> RouteHandler<RestService> {
    RouteHandler::new(self.method(), execute_transaction)
}
```

> LifeCycle part 2 starts here.

```Rust
/// Execute Transaction REST endpoint.
///
/// Handles client transaction submission request by passing off the provided signed transaction to
/// an internal QuorumDriver, which drives execution of the transaction with the current validator
/// set.
///
/// A client can signal, using the `Accept` header, the response format as either JSON or BCS.
/// Note: this is where the TX request starts.
async fn execute_transaction(
    State(state): State<Option<Arc<dyn TransactionExecutor>>>,
    Query(parameters): Query<ExecuteTransactionQueryParameters>,
    client_address: Option<axum::extract::ConnectInfo<SocketAddr>>,
    accept: AcceptFormat,
    Bcs(transaction): Bcs<SignedTransaction>,
) -> Result<ResponseContent<TransactionExecutionResponse>> {}
```

- In this func after some rust magic using Generics and Traits, the transaction is being executed by
  `execute_transaction_v3()`, located inside the `TransactiondOrchestrator` struct.

- `TransactiondOrchestrator` is a Node component that uses `QuorumDriver` to
  submit transactions to validators for finality and proactively executes
  finalized transactions locally, when possible.

- The committee of the validators for each Epoch is held by the `AuthorityAggregator` and is initialized by the
  `AuthorityAggregator::new_from_epoch_start_state()` function.

- The `QuorumDriver` holds an instance of the `AuthorityAggregator`.

```Rust
pub struct QuorumDriver<A: Clone> {
    validators: ArcSwap<AuthorityAggregator<A>>,
    task_sender: Sender<QuorumDriverTask>,
    effects_subscribe_sender: tokio::sync::broadcast::Sender<QuorumDriverEffectsQueueResult>,
    notifier: Arc<NotifyRead<TransactionDigest, QuorumDriverResult>>,
    metrics: Arc<QuorumDriverMetrics>,
    max_retry_times: u32,
}
```

- `NetworkAuthorityClient` is a client that sends transactions to the network, and it is being used by the
  `AuthorityAggregator` to send transactions to the network.

- The function that actually sends the data is called `quorum_map_then_reduce_with_timeout_and_prefs()` by assembling a
  set of functions and data, and executing them.
  It's called from `AuthorityAggregator::process_transaction()`.

- Based on the type of network (i.e., Real, Local, or Test), the `AuthorityAggregator::authority_clients` field is being
  initialized with the correct configuration.
  The trait that allows this is called `AuthorityAPI` and is implemented by the `NetworkAuthorityClient` struct.

```Rust
impl<A> AuthorityAggregator<A> where A: AuthorityAPI + Send + Sync + 'static + Clon {}
```

- The code for the Validator Client and Server is generated from the gRPC definition.
- The communication of `NetworkAuthorityClient` is done via gRPC using a the `tonic` crate.

```Rust
    async fn handle_transaction(
    &self,
    transaction: Transaction,
    client_addr: Option<SocketAddr>,
) -> Result<HandleTransactionResponse, PeraError> {
    let mut request = transaction.into_request();
    insert_metadata(&mut request, client_addr);

    self.client()?
        .transaction(request)
        .await
        .map(tonic::Response::into_inner)
        .map_err(Into::into)
}
```

### Validator Side — Process Transaction.

> LifeCycle part 2

- The Validator receivers the transaction via the gRPC function called `ValidatorService::transaction`.
- The `ValidatorService` is a struct that implements the `Validator` trait, which is generated by the gRPC compiler.
- The `ValidatorService::handle_transaction()` is the function that processes the transaction from the Validator side.
- Next step is `AuthorityState::handle_transaction()`, which validates the inputs, and objects and signs the TX.

### Full Node—Assemble the Certificate

> LifeCycle part 3

- The Full Node receives the signed transaction from the Validators, and assembles the certificate.
- This happens in the Reduce part of the `quorum_map_then_reduce_with_timeout()` called from
  `AuthorityAggregator::process_transaction()`
- Then the code goes back to `AuthorityAggregator::execute_transaction_block()`, which takes the certified transaction
  and sends it to the `AuthorityAggregator::process_certificate()` function.
  From here, the process of sending the certs involves the same flow as sending a transaction (see above),
  but the gRPC function is `NetworkAuthorityClient::handle_certificate_v3()`

### Validator Side—Process Certificate

> gRPC Server → Consensus → Primary → Worker

> LifeCycle part 4

- Certificates are received by the `ValidatorService::handle_certificate_v3()` function.
- This function sends the certificates to the consensus engine.
- All certificates are sent to consensus.
  For shared objects, this will wait until either timeout, or we have heard back from consensus.
  For owned objects, this will return without waiting for a certificate to be sequenced.
- The certificates are submitted to the consensus engine using the `ConsensusAdapter::submit_batch()` function,
- Which then calls  `ConsensusAdapter::submit_and_wait_inner()`, which eventually calls
  `SubmitToConsensus::submit_to_consensus()` trait,
  which is implemented by the different types of Consensus Engines that can be used in Production or tests
  (Implementations are: LazyNarwhalClient, LazyMysticetiClient, and for tests).
- This function calls the `LocalNarwhalClient::submit_transaction()` function,
  This transaction is submitted to the Worker by using a channel
  (the consensus has the concept of Primary and Workers).
- The `Worker::handle_clients_transactions()` spawns all relevant processes for handling the TX.
- There are two critical processes that are spawned:
    1. `BatchMaker` - Assemble clients' transactions into batches, this is where the data from the Primary is received.
    2. `QuorumWaiter` The QuorumWaiter waits for 2f authorities to acknowledge reception of a batch.
       It then forwards the batch to the `Processor`.
- Both communicate with channels.
- There is also a `Synchronizer` spawned by the primary the handles all the back and forth communication with the
  workers and the primary.

> `Synchronizer` helps this primary and other peers stay in sync with each other,
> w.r.t.
> Certificates and the DAG.
> Specifically, it handles:
> - Validating and accepting certificates received from peers.
> - Triggering fetching for certificates and batches.
> - Broadcasting created certificates.

> LifeCycle part 5

- After the whole consensus process was done, the consensus output is sent to the `Exectuor` service, which is a
  client subscribing to the consensus output and executing every transaction.
- Which eventually calls `ExeuctionState::handle_consensus_output()`.
- Which calls `ConsensusHandler::handle_consensus_output_internal()`, this is not executing the transactions yet, but
  does some preparations.
- Eventually this function calls: `self.transaction_scheduler.schedule(transactions_to_schedule).await`, which is an
  `AsyncTransactionScheduler` which wraps the `TransactionManager` that has a field:
  `tx_ready_certificates: UnboundedSender<PendingCertificate>`
- The `TransactionManager::enqueue_impl()` inserts the certificate to a pending for execution queue.
- The actual execution logic is inside `AuthorityState`.
  After a transaction commits and updates storage, committed objects and certificates are notified back to
  `TransactionManager`.
- The `TransactionManager` sends the ready certificates via a channel to `AuthorityState::execution_process()`.
- Which then calls `AuthorityState::try_execute_immediately()`.
- Which then calls `AuthorityState::process_certificate()`.
- Which then calls `AuthorityState::commit_certificate()`.

> Back to our flow

- The Move function emits a `StartDKGFirstRoundEvent` event as part of the transaction.
- When the validator executes the transaction in the `AuthorityState::commit_certificate()` function, it looks for any
  `StartDKGFirstRoundEvent` events in the `AuthorityState::filter_dwallet_mpc_events()` function.
  The `session_info_from_event()` function does that filtering, and it returns `Option::Some(data)` only if the event is
  a dWallet MPC related event.
- If it finds such an event, it sends it, using the `AuthorityPerEpochStore::dwallet_mpc_sender` channel, to the
  DWalletMPCManager.
- We send the messages to the manager using this channel, so it will be able to run the heavy MPC
  cryptography on a different thread.
  Without blocking the chain from processing other transactions in the meantime, such as faucet requests or token
  transfers.
- The `DWalletMPCManager` creates a new `DwalletMPCSession` with the given session_id and sender from the event, or push
  the event to the pending creation instances queue if it reached the limit of
  `DwalletMPCManager::max_active_mpc_sessions`.
  The limit is being set in the Validator’s configuration file.

# High Level Overview

### Connectivity

Sui Node uses the following ports by default:

| protocol/port | reachability     | name                      | purpose                                   | field in config                                                              |
|---------------|------------------|---------------------------|-------------------------------------------|------------------------------------------------------------------------------|
| TCP/8080      | inbound          | network_address           | protocol / transaction interface via gRPC | network-address: /ip4/0.0.0.0/tcp/8080/http                                  |
| TCP/8081      | inbound/outbound |                           | consensus interface                       |                                                                              |
| UDP/8081      | inbound/outbound | narwhal_primary_address   | narwhal primary interface                 |                                                                              |
| UDP/8082      | inbound/outbound | narwhal_worker_address    | narwhal worker interface                  |                                                                              |
| UDP/8084      | inbound/outbound | p2p-config.listen-address | peer to peer state sync interface         | p2p-config.listen-address: 0.0.0.0:8084                                      |
| TCP/8443      | outbound         | metrics.push-url          | metrics pushing                           | metrics.push-url: https://metrics-proxy.testnet.pera.io:8443/publish/metrics |
| TCP/9184      | localhost        | metrics-address           | metrics scraping                          | metrics-address: 0.0.0.0:9184                                                |

- Port 8084 is used to set the external address for the Node/Validator.
  The external address other nodes can use to reach the node.
  This will be shared with other peers through the discovery service.
