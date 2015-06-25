#![allow(dead_code)]
use std::marker::PhantomData;

/// Heap's algorithm for generating permutations,
/// recursive version.
fn heap<T, F>(xs: &mut [T], mut f: F) where F: FnMut(&mut [T])
{
    heap_(xs.len(), xs, &mut f);
}

fn heap_<T>(n: usize, xs: &mut [T], f: &mut FnMut(&mut [T]))
{
    if n <= 1 {
        f(xs);
        return;
    }
    for i in 0..n {
        heap_(n - 1, xs, f);
        let j = if n % 2 == 0 { i } else { 0 };
        xs.swap(j, n - 1);
    }
}

/// Maximum number of elements we can generate permutations for using
/// Heap's algorithm
pub const MAXHEAP: usize = 16;

/// Heap's algorithm for generating permutations.
///
/// An iterative method of generating all permutations of a sequence.
///
/// Note that for *n* elements there are *n!* (*n* factorial) permutations.
pub struct Heap<'a, Data: 'a + ?Sized, T: 'a> {
    data: &'a mut Data,
    c: [u8; MAXHEAP],
    n: u8,
    // we can store up to 20! in 64 bits.
    index: u64,
    _element: PhantomData<&'a mut T>
}

impl<'a, T, Data: ?Sized> Heap<'a, Data, T>
    where Data: AsMut<[T]>
{
    /// Create a new `Heap`.
    pub fn new(data: &'a mut Data) -> Self {
        assert!(data.as_mut().len() <= MAXHEAP);
        Heap {
            data: data,
            c: [0; MAXHEAP],
            n: 0,
            index: 0,
            _element: PhantomData,
        }
    }

    /// Return a mutable reference to the inner data
    pub fn get_mut(&mut self) -> &mut Data {
        self.data
    }

    /// Reset the permutations walker, and the order of the data.
    ///
    /// Note: This will run through all remaining permutations,
    /// which may be very many.
    fn reset_reverse(&mut self) {
        // spin to the end, then reset all variables
        while let Some(_) = self.next_permutation() { }
        self.n = 0;
        for c in &mut self.c[..] { *c = 0; }
        self.index = 0;
    }

    /// Step the internal data into the next permutation and return
    /// a reference to it. Return `None` when all permutations
    /// have been visited.
    ///
    /// Note that for *n* elements there are *n!* (*n* factorial) permutations.
    pub fn next_permutation(&mut self) -> Option<&mut Data> {
        if self.index == 0 {
            self.index += 1;
            Some(self.data)
        } else {
            while (self.n as usize) < self.data.as_mut().len() {
                let n = self.n;
                let nu = self.n as usize;
                let c = &mut self.c;
                if c[nu] < n {
                    // `n` acts like the current length - 1 of the slice we are permuting
                    // `c[n]` acts like `i` in the recursive algorithm
                    let j = if (n + 1) % 2 == 0 { c[nu] as usize } else { 0 };
                    self.data.as_mut().swap(j, nu);
                    c[nu] += 1;
                    self.n = 0;
                    return Some(self.data);
                } else {
                    c[nu] = 0;
                    self.n += 1;
                }
            }
            // Note: The last permutation visited has all elements in
            // reverse order from the starting configuration.
            None
        }
    }
}

/// Iterate the permutations
///
/// **Note:** You can also generate the permutations lazily by using
/// `.next_permutation()`.
impl<'a, Data: ?Sized, T> Iterator for Heap<'a, Data, T>
    where Data: AsMut<[T]> + ToOwned,
{
    type Item = Data::Owned;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_permutation() {
            None => None,
            Some(perm) => Some(perm.to_owned()),
        }
    }
}

/// Compute *n!* (*n* factorial)
pub fn factorial(n: usize) -> usize {
    let mut prod = 1;
    for x in 1..n + 1 { prod *= x; }
    prod
}

#[test]
fn first_and_reset() {
    let mut data = [1, 2, 3];
    let mut heap = Heap::new(&mut data);
    let mut perm123 = vec![[1, 2, 3], [2, 1, 3], [3, 1, 2], [1, 3, 2], [2, 3, 1], [3, 2, 1]];
    assert_eq!(heap.by_ref().collect::<Vec<_>>(), perm123);

    // test reset_reverse -- dubious
    heap.reset_reverse();
    perm123.reverse();
    assert_eq!(heap.by_ref().collect::<Vec<_>>(), perm123);
}

/*
#[test]
fn test_reset() {
    // test that `reset_reverse` restores the data in original order
    let mut data = [3, 1, 2, 4];
    let orig = data;
    for n in 0..factorial(data.len()) {
        {
            let mut heap = Heap::new(&mut data);
            for _ in 0..n {
                heap.next_permutation();
            }
            heap.reset_reverse();
            heap.reset_reverse();
        }
    }
}
*/

#[test]
fn permutations_0_to_5() {
    let mut data = [0; 6];
    for n in 0..data.len() {
        let count = factorial(n);
        for (index, elt) in data.iter_mut().enumerate() {
            *elt = index + 1;
        }
        let mut permutations = Heap::new(&mut data[..n]).collect::<Vec<_>>();
        assert_eq!(permutations.len(), count);
        //println!("{:?}", permutations);
        permutations.sort();
        permutations.dedup();
        assert_eq!(permutations.len(), count);
        // Each permutation contains all of 1 to n
        assert!(permutations.iter().all(|perm| perm.len() == n));
        assert!(permutations.iter().all(|perm| (1..n + 1).all(|i| perm.iter().position(|elt| *elt == i).is_some())));
    }
}
