use clap::Subcommand;
use sui_types::base_types::ObjectID;

#[derive(Subcommand)]
pub enum LightClientCommands {
    /// Approve a TX with Eth contract for a given dWallet.
    #[command(name = "dwallet-eth-verify")]
    EthApproveMessage {
        #[clap(long)]
        /// Object of a [EthDwalletCap]
        eth_dwallet_cap_id: ObjectID,
        /// The Message (TX) to verify.
        #[clap(long)]
        message: String,
        /// The DWallet ID
        // todo(zeev): in the future we can fetch the dwallet ID.
        #[clap(long)]
        dwallet_id: ObjectID,
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
        #[clap(long)]
        gas_budget: u64,
        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,
        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },

    /// Connect dWallet to be controlled by Eth contract.
    #[command(name = "dwallet-connect-eth")]
    CreateEthDwallet {
        /// The ObjectID of the dWallet *cap*ability.
        #[clap(long)]
        dwallet_cap_id: ObjectID,
        /// The address of the contract.
        #[clap(long)]
        smart_contract_address: String,
        /// The slot of the Data structure that holds approved transactions in eth smart contract.
        #[clap(long)]
        // todo(zeev): change the clap name to something shorter.
        smart_contract_approved_tx_slot: u64,
        /// The address of the gas object for gas payment.
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
        #[clap(long)]
        gas_budget: u64,
        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,
        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },

    /// Initiate the LatestEthereumState struct in the DWallet module.
    #[command(name = "init-eth-state")]
    InitEthState {
        #[clap(long)]
        checkpoint: String,
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
        #[clap(long)]
        gas_budget: u64,
        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,
        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    }
}
