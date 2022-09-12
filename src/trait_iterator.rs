/*
 * File: trait_iterator.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub trait Iterator {
    // You have to define Item type
    type Item;
    // You have to implement next()
    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementations elided
}

#[cfg(test)]
mod tests {
    use std::iter::Iterator;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn mut_iterator_demonstration() {
        let mut v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter_mut();
        assert_eq!(v1_iter.next(), Some(&mut 1));
        assert_eq!(v1_iter.next(), Some(&mut 2));
        assert_eq!(v1_iter.next(), Some(&mut 3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn owned_iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.into_iter();
        assert_eq!(v1_iter.next(), Some(1));
        assert_eq!(v1_iter.next(), Some(2));
        assert_eq!(v1_iter.next(), Some(3));
        assert_eq!(v1_iter.next(), None);
    }

    /*
     * There is various methods for iterators
     * Categories:
     * - adapters -> take iterator, return iterator
     * - consumers -> take iterator, return sth else
     */
    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        // adapts iterator; creates iterator which calls the closure in each sequence
        let v1_iter_map = v1_iter.map(|x| x + 1);
        let v1_iter_map_collection: Vec<_> = v1_iter_map.collect();
        assert_eq!(v1_iter_map_collection, vec![2, 3, 4]);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        // consumes iterator
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }
}
