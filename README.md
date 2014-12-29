sort
====

crate sort is a collection of Rust function to demonstrate the use of
random testing. Here, I show the usage of simple random number generator
just to show what one 
Read

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

