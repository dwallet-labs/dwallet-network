## MPC Notes

- Aggregation function is a part of the MPC code, not related to 2PCMPC.
- User -> BC -> MPC -> BC -> User
- DKG:
- Presign:
    - User
    - Blockchain
    - Output
    - User takes output for Sign
    - Blockchain finishes presign
    - User sends sign to blockchain with all outputs.

## Packages

- SUI-Framework is the Move modules.
- [dwallet.move](crates/sui-framework/packages/dwallet-system/sources/dwallet.move)
- [dwallet_2pc_mpc_ecdsa_k1.move](crates/sui-framework/packages/dwallet-system/sources/dwallet_2pc_mpc_ecdsa_k1.move)
- Inside `signature_mpc` dir the file `aggregate.rs` is used for inner aggregation.
- Each Protocol (DKG, Presign, Sign) has its own module, but it calls aggregation function to do inner block
  aggregation (part of the MPC requirements)
- Currently all validators are required, there is no threshold.
