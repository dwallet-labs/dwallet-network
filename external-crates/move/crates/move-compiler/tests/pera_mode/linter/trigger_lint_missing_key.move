module a::trigger_lint_cases {
    use pera::object::UID;

    // This should trigger the linter warning (true positive)
    struct MissingKeyAbility {
        id: UID,
    }

}

module pera::object {
    struct UID has store {
        id: address,
    }
}
