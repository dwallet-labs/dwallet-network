module a::trigger_lint_cases {
    use ika::object::UID;

    // 4. Suppress warning
    #[allow(lint(missing_key))]
    struct SuppressWarning {
       id: UID,
    }
}

module ika::object {
    struct UID has store {
        id: address,
    }
}
