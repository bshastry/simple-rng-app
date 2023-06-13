#### Build

```
$ cargo build
```

#### Test

```
$ cargo test
...
running 2 tests
test tests::test_without_seed ... ok
test tests::test_with_seed ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### Run

```
$ cargo run -- -s 1337
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/simple-rng-app -s 1337`
The seed is 1337.
The random number is: 12090749038398272714
```
