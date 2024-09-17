module a::edge_cases {
    struct UID {}
    // Test case with a different UID type
    struct DifferentUID {
        id: pera::another::UID,
    }

    struct NotAnObject {
        id: UID,
    }

}

module pera::object {
    struct UID has store {
        id: address,
    }
}

module pera::another {
    struct UID has store {
        id: address,
    }
}
