use dwallet_mpc::create_dkg_output;

fn main() {
    let hex_to_bytes = "001df78f3b5736463644764f313d47b3c0733cb930f710346db706bcd254fc8f08cf84d4723fc4cd26966a3bcbab2c576aefbbca0348b01e54a9b4988fde14c43517d0cba537131cc097bad5fdd1d86796e9a9c144b4c89e9c705a523c25a9cf830e4e6fb83e08ed77c18024ab2660eb6d8888e0d00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008bcfa77de688a288bbc23439d153759de5e3d4416d1fbf088e615bbc4075597b4b5e433bc393ceadd39126d90da86f04a914d1abe98fe0479b9322cedf84852092cfccc82df3bdc67cdb26139a45a3fedcdc8b54c6d2c43e6c86b4bf2ec16b63fb3db2f9f99b374616f50895deea4e62c68026a700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000037237c2d9edbfdeae60ec990c5afffae8239568839daab863dbab5bb64da077b74a7f3140ba1b97510246442ac3a46a2bbc09462c2ce7ba6345a07f0884bc0fb9e90ffa418f2e00480af64375ac4b24ef09d8c848c7f0641af61d0b2aafe219ac41a9b5ae4e670bbb50949c98aedcb7d660980530100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005f8eb4ba8e6b747bb7180363a91ffd661ffeaae5969868d06198ad7d34d348d5dce52a6035dfc0de055178b0781a4e7de7686a48898851977228465fb9679961111d90b4e98d391d6db41ae0c12b02be2d81bab71ba18364a88d9a121acd589c372d5b6de6a86b4d45ec6c7b87b23e7048be708f00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002103d6bc7b6e845778b4cc902f4b70d09d7a3c79b9a1e05c8f04679535b9ffb64d5f";
    let output = hex::decode(hex_to_bytes).unwrap();
    println!("{:?}", output);
    let a = create_dkg_output(
        output,
        "75d5a37024737affcc58877228614d88fd7bc3782bbf747738a68a3e5133ec68".to_string(),
    );
    println!("{:?}", a);
}
