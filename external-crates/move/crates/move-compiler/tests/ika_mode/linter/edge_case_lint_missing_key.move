module a::edge_cases {
    struct UID {}
    // Test case with a different UID type
    struct DifferentUID {
        id: ika::another::UID,
    }

    struct NotAnObject {
        id: UID,
    }

}

module ika::object {
    struct UID has store {
        id: address,
    }
}

module ika::another {
    struct UID has store {
        id: address,
    }
}
