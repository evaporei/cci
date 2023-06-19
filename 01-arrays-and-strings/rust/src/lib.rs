use std::collections::HashSet;

// 1.1 Is Unique: Implement an algorithm to determine if a string has all unique characters.
// What if you cannot use additional data structures?
// Hints: #44, #7 7 7, #732
//
// time complexity:
// s = s.len
// m = possible characters
// O(s*m)
//
// space complexity:
// s = s.len
// O(s)
// at most the amount of characters in s
// that is, s times amount of bytes for a char in UTF-8?
pub fn is_unique(s: &str) -> bool {
    let mut used_chars = HashSet::new();
    for ch in s.chars() { // O(s)
        if !used_chars.contains(&ch) { // worst case O(n)
            used_chars.insert(ch); // "O(1)" -> amortized
        } else {
            return false;
        }
    }
    true
}

// no extra data structures
// assume from 'a' to 'z'
pub fn is_unique_bit(s: &str) -> bool {
    let mut checker = 0;
    for ch in s.chars() {
        let val = ch as i32 - 'a' as i32;
        if checker & (1 << val) > 0 {
            return false;
        }
        checker |= 1 << val;
    }
    true
}

#[test]
fn test_is_unique() {
    assert!(!is_unique("aabb"));
    assert!(is_unique("asdf"));

    assert!(!is_unique_bit("aabb"));
    assert!(is_unique_bit("asdf"));
}

// 1.2 Check Permutation: Given two strings, write a method to decide if one is a permutation of the
// other.
// Hints: #7, #84, #722, #737
//
// time complexity:
// a = b = n
// O(n + log n)
// or just O(n)?
pub fn check_permutation(a: &str, b: &str) -> bool {
    if a.len() != b.len() { // O(1)
        return false;
    }

    let mut chars_a = a.chars().collect::<Vec<char>>(); // O(a)
    chars_a.sort_by(|c1, c2| c1.cmp(c2)); // O(log a)
    let sorted_a: String = chars_a.into_iter().collect(); // O(a) -> doesn't count

    let mut chars_b = b.chars().collect::<Vec<char>>(); // O(b)
    chars_b.sort_by(|c1, c2| c1.cmp(c2)); // O(log b)
    let sorted_b: String = chars_b.into_iter().collect(); // O(b) -> doesn't count

    sorted_a == sorted_b // O(n) -> doesn't count
}

use std::collections::HashMap;

// a = b = n
// O(n)
pub fn check_permutation_char_count(a: &str, b: &str) -> bool {
    if a.len() != b.len() { // O(1)
        return false;
    }

    let mut letters = HashMap::new();

    for ch in a.chars() { // O(a)
        letters.entry(ch) // O(1) amortized
            .and_modify(|count| *count += 1) // O(1) amortized
            .or_insert(1); // O(1) amortized
    }


    for ch in b.chars() { // O(b)
        letters.entry(ch) // O(1) amortized
            .and_modify(|count| *count -= 1) // O(1) amortized
            .or_insert(-1); // O(1) amortized

        if *letters.get(&ch).unwrap() < 0 { // O(1) amortized
            return false;
        }
    }

    true
}

#[test]
fn test_check_permutation() {
    assert!(check_permutation("abc", "cab"));
    assert!(!check_permutation("abc", "zef"));
    assert!(!check_permutation("abc", "abj"));
    assert!(!check_permutation("abcd", "abc"));

    assert!(check_permutation_char_count("abc", "cab"));
    assert!(!check_permutation_char_count("abc", "zef"));
    assert!(!check_permutation_char_count("abc", "abj"));
    assert!(!check_permutation_char_count("abcd", "abc"));
}
