module a::trigger_lint_cases {
    use ika::object::UID;

    // This should trigger the linter warning (true positive)
    struct MissingKeyAbility {
        id: UID,
    }

}

module ika::object {
    struct UID has store {
        id: address,
    }
}
