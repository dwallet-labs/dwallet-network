use dwallet_classgroups_types::read_class_groups_from_file;
use dwallet_classgroups_types::ClassGroupsKeyPairAndProof;

pub fn create_full_class_groups_mock() -> Box<ClassGroupsKeyPairAndProof> {
    include_str!("../../../class-groups-keys-mock-files/class-groups-mock-key-full");
    let file_name = "class-groups-keys-mock-files/class-groups-mock-key-full".to_string();
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join(file_name);
    // Safe to unwrap because the file is used for development purposes and should never fail
    read_class_groups_from_file(file_path).unwrap()
}
