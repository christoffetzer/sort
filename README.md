sort
====

crate sort is a collection of Rust function to demonstrate the use of
random testing and post-conditions. It should show that post-conditions
are not always easy to generate. Checking if result is sorted is
insufficient - we also need to test if it is a permutation.

### Installation

To add sort to your package, just add the following
dependency.

```toml
[dependencies.sort]
git = "https://github.com/christoffetzer/sort"
```

The `sort` crate depends on the two other crates:

- rndtester: https://github.com/christoffetzer/rndtester

	a very simple crate to help with writing
	random test cases. This only demonstrate how random
	test cases could be programed manually. 

- quickcheck: https://github.com/BurntSushi/quickcheck

### Test

Test the code by pulling it with cargo or git and execute:

```sh
cargo test
```

### Documentation

Build the documentation by running

```sh
cargo doc
```
