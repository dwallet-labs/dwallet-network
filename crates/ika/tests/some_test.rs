use dwallet_classgroups_types::read_class_groups_from_file;

#[test]
fn test_class_groups_deserialize() {
    let _ = read_class_groups_from_file("/Users/yaelabergel/projects/dwallet_labs/dwallet-network-original/class-groups.key");
}