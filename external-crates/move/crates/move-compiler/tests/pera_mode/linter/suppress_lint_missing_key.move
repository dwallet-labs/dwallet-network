module a::trigger_lint_cases {
    use pera::object::UID;

    // 4. Suppress warning
    #[allow(lint(missing_key))]
    struct SuppressWarning {
       id: UID,
    }
}

module pera::object {
    struct UID has store {
        id: address,
    }
}
