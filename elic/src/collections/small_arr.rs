
use std::{fmt::Debug, ops::{Index, IndexMut}};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SmallArr<T: Copy + Default, const N: usize> {
    len: usize,
    elems: [T; N]
}

impl<T: Copy + Default, const N: usize> SmallArr<T, N> {

    pub fn new(len: usize) -> Self {
        assert!(len <= N);
        Self {
            len,
            elems: [T::default(); N],
        }
    }

    pub fn from_slice(elems: &[T]) -> Self {
        assert!(elems.len() <= N);
        let mut arr = Self::new(elems.len());
        arr.elems[0..elems.len()].copy_from_slice(elems);
        arr
    }

    pub fn empty() -> Self {
        Self::new(0)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_slice(&self) -> &[T] {
        &self.elems[0..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.elems[0..self.len]
    } 

    pub fn push(&mut self, elem: T) {
        assert!(self.len < N);
        self.elems[self.len] = elem;
        self.len += 1;
    }

    pub fn concat<const O: usize, const M: usize>(&self, other: &SmallArr<T, M>) -> SmallArr<T, O> {
        assert!(O >= N + M);
        let mut elems = [T::default(); O];
        elems[0..self.len].copy_from_slice(self.as_slice());
        elems[self.len..(self.len + other.len)].copy_from_slice(other.as_slice());
        SmallArr { len: self.len + other.len, elems }
    }

    pub fn map<R: Copy + Default, F: Fn(T) -> R>(&self, map: F) -> SmallArr<R, N> {
        let mut elems = [R::default(); N];    
        for i in 0..self.len {
            elems[i] = map(self.elems[i]);
        }
        SmallArr {
            len: self.len,
            elems
        }
    }

    pub fn filter<F: Fn(T) -> bool>(&self, filter: F) -> Self {
        let mut new_arr = Self::empty();
        for elem in self.as_slice() {
            if filter(*elem) {
                new_arr.push(*elem);
            }
        }
        new_arr
    }

    pub fn retain<F: Fn(T) -> bool>(&mut self, filter: F) {
        *self = self.filter(filter); 
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.elems[0..self.len].iter().copied()
    }

}

impl<T: Default + Copy + PartialEq, const N: usize> SmallArr<T, N> {

    pub fn swap_remove_item(&mut self, item: T) {
        for i in 0..self.len() {
            if self[i] == item {
                self[i] = self[self.len() - 1];
                self.len -= 1;
                return;
            }
        }
    }

}

impl<T: Default + Copy + Debug, const N: usize> Debug for SmallArr<T, N> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }

}

impl<T: Copy + Default, const N: usize> Index<usize> for SmallArr<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T: Copy + Default, const N: usize> IndexMut<usize> for SmallArr<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}
