#![experimental]

#![feature(phase)]
#[phase(plugin)]
extern crate quickcheck_macros;
extern crate quickcheck;


// (C) Christof Fetzer, 2014


#[doc = "
Check if array is sorted using splicing.

Problem: This implementation uses slicing. However, slicing inside
the match does not seem to work for generic types due to limitation
of Rust. Hence, we use a more 'operational' function is_sorted instead.

Example usage:

```test_harness
#[test]
 
fn test_is_sorted_int() {	
     let sorted = [1i,4,5];
     let notsorted = [1i,5,4];
     let empty : [int, ..0] = [];
     assert!(sort::is_sorted_int(&sorted));
     assert!(!sort::is_sorted_int(&notsorted));
     assert!(sort::is_sorted_int(&empty));
}
```
"]
pub fn is_sorted_int(array: &[int]) -> bool {
	match array {
		[] | [_] => true,  // sorted if only 0 or 1 elements
		[x, right..] if x <= right[0]  => is_sorted_int(right),
		_ => false         // detected that we are out of order
	}
}

#[doc = "
Returns true iff given slice is sorted

Function is_sorted checks if argument 'a' is sorted. 
For argument 'a', operator '>' must be defined. 

```test_harness
#[test]

fn test_is_sorted() {	
    let sorted = [1i,4,5];
	let notsorted1 = [1i,5,4];
	let notsorted2 = [6i,5,4];
	let notsorted3 = [2i,7,5,6];
    let empty : [int, ..0] = [];
	assert!(sort::is_sorted(&sorted));
	assert!(!sort::is_sorted(&notsorted1));
	assert!(!sort::is_sorted(&notsorted2));
	assert!(!sort::is_sorted(&notsorted3));
	assert!(sort::is_sorted(&empty));
}
```
"]

pub fn is_sorted<T: std::cmp::PartialOrd>(a: &[T]) -> bool {
	for i in range(1u, a.len()) {
		if a[i-1] > a[i] { return false }
	}
	true
}


#[doc = "
Returns true iff there are no duplicates in the given slice

Iterates through slice 'a' to check if there exists at least 
another element in 'a' that is equivalent. If this exists,
it returns false and otherwise, true. 

```test_harness
#[test]

fn test_is_unique() {	
    let unique = [1i,4,5];
	let notunique = [1i,5,5];
	let notunique2 = [1i, 1, 5, 6];
    let empty : [int, ..0] = [];
	assert!(sort::is_unique(&unique));
	assert!(!sort::is_unique(&notunique));
	assert!(!sort::is_unique(&notunique2));
	assert!(sort::is_unique(&empty));
}
```
"]


pub fn is_unique<T: std::cmp::PartialEq>(array: &[T]) -> bool {
	for i in range(1,array.len()) {
		for j in range(i,array.len()) {
			if array[i-1] == array[j] { return false }
		}
	}
	true
}

#[doc = "
returns true iff the two slices are permutations of each other

tests if slice 'a' and 'b' are permutations of each other
by checking that (1) that 'a' and 'b' have the same length and for each element
in 'a' there exists exactly one equivalent element in slice 'b'.

```test_harness
#[test]

fn test_is_permutation() {	
    let perm1 = [1i, 4, 5];
	let same_as_perm1 = [5i, 1, 4];
	let perm2 = [5i, 1, 5];
    let empty1 : [int, ..0] = [];
    let empty2 : [int, ..0] = [];
	assert!(sort::is_permutation(&perm1, &perm1));
	assert!(sort::is_permutation(&perm1, &same_as_perm1));
	assert!(!sort::is_permutation(&perm1, &perm2));
	assert!(!sort::is_permutation(&perm2, &same_as_perm1));
	assert!(sort::is_permutation(&empty1, &empty2));
	assert!(!sort::is_permutation(&empty1, &perm1));
	assert!(!sort::is_permutation(&empty2, &same_as_perm1));
}
```
"]

pub fn is_permutation<T: std::cmp::PartialEq>(a: &[T], b: &[T]) -> bool {
	let mut used = Vec::from_fn(b.len(), |_| false);
	let mut count = 0u;
	 
	for i in range(0, a.len()) {
		for j in range(0, b.len()) {
			if a[i] == b[j] && !used[j] { 
				used[j] = true; count +=1; break;
			}
		}
	}
	a.len() == b.len() && count == a.len()
}


#[doc = "
sorts a given slice and merging it in a given vector. 

sorts slice 'a' into vector 'b' and returns 'b' which is a sorted
permutation of 'a'. If 'b' is not empty, 'a' will be merged with 'b'

```test_harness
extern crate sort;
extern crate rndtester;

use rndtester::RndTester;
use rndtester::rnd_values;

#[test]
fn test_sort() {	
    let a = [1i,5,4];
	let b = [1i,4,4,55,6,7];
	let mut vv: Vec<int> = Vec::with_capacity(a.len());
	vv = sort::sort(&a, vv);
	assert!(sort::is_sorted(vv.as_slice()));
	assert!(sort::is_permutation(&a,vv.as_slice()));
	// note: we should clean vv in sort - bug that we do not want to detect now
	let mut w: Vec<int> = Vec::with_capacity(a.len());
	w = sort::sort(&b, w);
	assert!(sort::is_sorted(w.as_slice()));
	assert!(sort::is_permutation(&b,w.as_slice()));

    let (mut it, mut v) = rndtester::rnd_values();

    for i in range(0u,10) {
		let vi : Vec<i32> = v.rnd_vec();
		let mut vo: Vec<i32> = Vec::with_capacity(vi.len());

        vo = sort::sort(vi.as_slice(), vo);
		assert!(sort::is_sorted(vo.as_slice()));
		assert!(sort::is_permutation(vi.as_slice(),vo.as_slice()));
    }
	
}
```
"]

pub fn sort<T: std::cmp::PartialOrd+Clone>(a: &[T], mut v : Vec<T>) -> Vec<T>  {
    let mut m : uint;

	for i in range(0, a.len()) {
		m = v.len();
		for j in range(0, v.len()) {
			if v[j] > a[i] { m = j; break; }
		}
		v.insert(m, a[i].clone());
	}
	v
}

#[quickcheck]
fn check_sort(xs: Vec<int>) -> bool {
	let mut vo: Vec<int> = Vec::with_capacity(xs.len());

    vo = sort(xs.as_slice(), vo);
	is_sorted(vo.as_slice()) 
	 && is_permutation(xs.as_slice(),vo.as_slice())
	
}


