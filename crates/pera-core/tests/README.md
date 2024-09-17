## What this test is about 

This repository checks the conformance of our code to a BCS-compatible manifest of our serialized data formats.

It does this by running a manifest generator from the code (using serde-reflection) and checking the output has not changed.

If it has in a legitimate fashion (e.g. we update one of our main types), all that's left to do is to re-run the generator and check in the change.

Here are the references to the software above:
https://github.com/diem/bcs
https://github.com/novifinancial/serde-reflection

## Examples

In this example, we will update one of our core types (PeraError), and then update the manifest:

```
huitseeker@Garillots-MBP.localdomain➜~/tmp/pera(main)» git checkout main                                                                                                                                                                                                                                                                                                                                                                                                               [7:40:40]
Already on 'main'
Your branch is up to date with 'origin/main'.
huitseeker@Garillots-MBP.localdomain➜~/tmp/pera(main)» ruplacer --subvert 'CertificateAuthorityReuse' 'CertificateAuthorityDuplicate' --go                                                                                                                                                                                                                                                                                                                                             [8:42:33]
./pera_types/src/error.rs:103 - CertificateAuthorityReuse,
./pera_types/src/error.rs:103 + CertificateAuthorityDuplicate,

./pera_types/src/messages.rs:610 - PeraError::CertificateAuthorityReuse
./pera_types/src/messages.rs:610 + PeraError::CertificateAuthorityDuplicate
./pera_types/src/messages.rs:638 - PeraError::CertificateAuthorityReuse
./pera_types/src/messages.rs:638 + PeraError::CertificateAuthorityDuplicate

./pera_core/tests/staged/pera.yaml:390 - CertificateAuthorityReuse: UNIT
./pera_core/tests/staged/pera.yaml:390 + CertificateAuthorityDuplicate: UNIT

Performed 4 replacements on 196 matching files
```

Now our code is modified in a way that will make the format test fail: let's update the manifest.

```
huitseeker@Garillots-MBP.localdomain➜~/tmp/pera(main✗)» cd pera_core                                                                                                                                                                                                                                                                                                                                                                                                                    [8:43:38]
huitseeker@Garillots-MBP.localdomain➜tmp/pera/pera_core(main✗)» cargo -q run --example generate-format -- print > tests/staged/pera.yaml
```


Let's check that we pass the test again:
```
huitseeker@Garillots-MBP.localdomain➜tmp/pera/pera_core(main✗)» cargo test format 2>&1 |tail -n 40                                                                                                                                                                                                                                                                                                                                                                                      [8:47:22]
    Finished test [unoptimized + debuginfo] target(s) in 0.35s
     Running unittests (/Users/huitseeker/tmp/pera/target/debug/deps/pera_core-5796871991341984)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

     Running tests/format.rs (/Users/huitseeker/tmp/pera/target/debug/deps/format-ecdfa91a67810be3)

running 1 test
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/examples/generate-format test`
test test_format ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s
huitseeker@Garillots-MBP.localdomain➜tmp/pera/pera_core(main✗)» git status -s                                                                                                                                                                                                                                                                                                                                                                                                           [8:47:38]
 M tests/staged/pera.yaml
 M ../pera_types/src/error.rs
 M ../pera_types/src/messages.rs
 ```