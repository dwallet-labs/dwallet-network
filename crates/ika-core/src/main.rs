use dwallet_classgroups_types::ClassGroupsKeyPairAndProof;
use dwallet_rng::RootSeed;

fn main() {
    let root_seed = base64::decode("Nq6w6BGFA9cSIPYmbQY0Ar+IdKWin8N//0LeFDI0Qik=").unwrap();
    let class_groups_key_pair =
        ClassGroupsKeyPairAndProof::from_seed(&RootSeed::new(root_seed.try_into().unwrap()));
    println!("{:?}", class_groups_key_pair);
}
