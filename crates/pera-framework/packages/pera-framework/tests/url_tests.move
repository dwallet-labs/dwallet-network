// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
module pera::url_tests {
    use pera::url;

    const EUrlStringMismatch: u64 = 1;

    #[test]
    fun test_basic_url() {
        // url strings are not currently validated
        let url_str = x"414243454647".to_ascii_string();

        let url = url::new_unsafe(url_str);
        assert!(url::inner_url(&url) == url_str, EUrlStringMismatch);
    }
}
