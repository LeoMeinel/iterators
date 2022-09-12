/*
 * File: iterators.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * iterators gives ability to iterate over a sequence of elements (of any storage type -> array, ...)
 * iterators encapsulate the logic for its functionality
 */

pub(crate) fn iterators() {
    iterate_over_vector();
}

fn iterate_over_vector() {
    let v1 = vec![1, 2, 3];
    // iterators are lazy, meaning it isn't created until it is used
    let v1_iter = v1.iter();
    // iterator isn't needed here, you could also just use v1 and it would use the same logic
    for value in v1_iter {
        println!("Got: {}", value);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // closure of .filter() will be called on each element, if true include element
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_my_size(shoes, 13);
        assert_eq!(
            in_my_size,
            vec![Shoe {
                size: 13,
                style: String::from("sandal"),
            },]
        );
    }
}
