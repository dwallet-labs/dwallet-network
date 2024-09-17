// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

mod utils;
use utils::setup_for_read;

// This example connects to the Pera testnet
// and collects information about the stakes in the network,
// the committee information,
// lists all the validators' name, description, and pera address,
// and prints the reference gas price.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (pera, active_address) = setup_for_read().await?;

    // ************ GOVERNANCE API ************ //

    // Stakes
    let stakes = pera.governance_api().get_stakes(active_address).await?;

    println!(" *** Stakes ***");
    println!("{:?}", stakes);
    println!(" *** Stakes ***\n");

    // Committee Info
    let committee = pera.governance_api().get_committee_info(None).await?; // None defaults to the latest epoch

    println!(" *** Committee Info ***");
    println!("{:?}", committee);
    println!(" *** Committee Info ***\n");

    // Latest Pera System State
    let pera_system_state = pera.governance_api().get_latest_pera_system_state().await?;

    println!(" *** Pera System State ***");
    println!("{:?}", pera_system_state);
    println!(" *** Pera System State ***\n");

    // List all active validators

    println!(" *** List active validators *** ");
    pera_system_state
        .active_validators
        .into_iter()
        .for_each(|validator| {
            println!(
                "Name: {}, Description: {}, PeraAddress: {:?}",
                validator.name, validator.description, validator.pera_address
            )
        });

    println!(" *** List active validators ***\n");
    // Reference Gas Price
    let reference_gas_price = pera.governance_api().get_reference_gas_price().await?;

    println!(" *** Reference Gas Price ***");
    println!("{:?}", reference_gas_price);
    println!(" *** Reference Gas Price ***\n");

    // ************ END OF GOVERNANCE API ************ //
    Ok(())
}
