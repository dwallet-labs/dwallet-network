module a::trigger_lint_cases {
    use ika::object::UID;

    // False negative cases (should trigger warning but might not):

    // 1. Different field name
    struct FN1_MissingKeyWithDifferentFieldName {
        uid: UID,
    }

    // 2. UID field not first
    struct FN2_MissingKeyUIDNotFirst {
        point: u64,
        id: UID,
    }

    // 3. Nested UID
    struct FN3_MissingKeyNestedUID {
        wrapper: Wrapper,
    }

    struct Wrapper {
        id: UID,
    }
}

module ika::object {
    struct UID has store {
        id: address,
    }
}
