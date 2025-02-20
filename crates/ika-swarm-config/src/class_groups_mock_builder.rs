use dwallet_classgroups_types::read_class_groups_from_file;
use dwallet_classgroups_types::ClassGroupsKeyPairAndProof;

pub fn create_full_class_groups_mock() -> Box<ClassGroupsKeyPairAndProof> {
    include_str!("../../../class-groups-keys-mock-files/class-groups-mock-key-full");
    let file_name = format!("class-groups-keys-mock-files/class-groups-mock-key-full");
    // Safe to unwrap because the file is used for development purposes and should never fail
    read_class_groups_from_file(file_name).unwrap()
}
