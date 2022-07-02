/*
 * iterators is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/iterators/blob/main/LICENSE
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
            vec![
                Shoe {
                    size: 13,
                    style: String::from("sandal"),
                },
            ]
        );
    }
}