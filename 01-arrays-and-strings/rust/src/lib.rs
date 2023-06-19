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

// URLify: Write a method to replace all spaces in a string with '%20'. You may assume that the string
// has sufficient space at the end to hold the additional characters, and that you are given the "true"
// length of the string. (Note: If implementing in Java, please use a character array so that you can
// perform this operation in place.)
// EXAMPLE
// Input: "Mr John Smith ", 13
// Output: "Mr%20John%20Smith"
// Hints: #53, #118
//
// time complexity:
// O(n)
pub fn urlify(s: &str) -> String {
    let mut out = String::with_capacity(s.len());

    for ch in s.chars() {
        if ch == ' ' {
            out.push_str("%20");
        } else {
            out.push(ch);
        }
    }

    out
}

pub fn urlify_in_place(s: &mut Vec<u8>, true_len: usize) {
    let mut space_count = 0;
    for ch in s.iter() {
        if *ch == b' ' {
            space_count += 1;
        }
    }

    let mut index = true_len + space_count * 2;
    // // set true end character as \0 in trash languages
    // s[true_len] = '\0';
    let mut i = true_len as isize - 1;
    while i >= 0 {
        if s[i as usize] == b' ' {
            s[index - 1] = b'0';
            s[index - 2] = b'2';
            s[index - 3] = b'%';
            index -= 3;
        } else {
            s[index - i as usize] = s[i as usize];
            index -= 1;
        }
        i -= 1;
    }
}

#[test]
fn test_urlify() {
    assert_eq!(urlify("Mr John Smith "), "Mr%20John%20Smith%20");
    assert_eq!(urlify("MrJohnSmith"), "MrJohnSmith");

    // use std::io::Write;
    //
    // let mut w_space: Vec<u8> = Vec::with_capacity(14 + 6);
    // w_space.write(b"Mr John Smith ").unwrap();
    // let true_len = w_space.len();
    // urlify_in_place(&mut w_space, true_len);
    // assert_eq!(w_space, b"MrJohnSmith");
    //
    // let mut wout_space: Vec<u8> = Vec::with_capacity(11);
    // wout_space.write(b"MrJohnSmith").unwrap();
    // let true_len = wout_space.len();
    // urlify_in_place(&mut wout_space, true_len);
    // assert_eq!(wout_space, b"MrJohnSmith");
}
