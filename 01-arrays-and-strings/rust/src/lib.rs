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

// 1.3 URLify: Write a method to replace all spaces in a string with '%20'. You may assume that the string
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

// 1.4 Palindrome Permutation: Given a string, write a function to check if it is a permutation of a palindrome.
// A palindrome is a word or phrase that is the same forwards and backwards.
// A permutation is a rearrangement of letters.
// The palindrome does not need to be limited to just dictionary words.
// EXAMPLE
// Input: Tact Coa
// Output: True (permutations: "taco cat", "atco eta", etc.)
// Hints: #106, #121, #134, #136
//
// time complexity:
// O(n)
pub fn palindrome_perm(s: &str) -> bool {
    let mut table = HashMap::new();

    for ch in s.chars().filter(|ch| ch.is_alphanumeric()).map(|ch| ch.to_ascii_lowercase()) {
        table.entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut found_odd = false;

    // I find this slightly disgusting
    for count in table.values() {
        if count % 2 == 1 {
            if found_odd {
                return false;
            }
            found_odd = true;
        }
    }

    found_odd
}

pub fn palindrome_perm2(s: &str) -> bool {
    let mut table = HashMap::new();
    let mut odd_count = 0;

    for ch in s.chars().filter(|ch| ch.is_alphanumeric()).map(|ch| ch.to_ascii_lowercase()) {
        table.entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        if table.get(&ch).unwrap() % 2 == 1 {
            odd_count += 1;
        } else {
            odd_count -= 1;
        }
    }

    odd_count <= 1
}

pub fn palindrome_perm3(s: &str) -> bool {
    let mut bit_vector = 0;

    for index in s.chars().filter(|ch| ch.is_alphanumeric()).map(|ch| ch.to_ascii_lowercase()).map(|ch| ch as u32) {
        if let Some(mask) = 1i32.checked_shl(index) {
            if bit_vector & mask == 0 {
                bit_vector |= mask;
            } else {
                bit_vector &= mask;
            }
        }
    }

    bit_vector == 0 || bit_vector & (bit_vector - 1) == 0
}

#[test]
fn test_palindrome_perm() {
    assert!(palindrome_perm("Tact Coa"));
    assert!(palindrome_perm2("Tact Coa"));
    assert!(palindrome_perm3("Tact Coa"));
}

// 1.5 One Away: There are three types of edits that can be performed on strings: insert a character,
// remove a character, or replace a character. Given two strings, write a function to check if they are
// one edit (or zero edits) away.
// EXAMPLE
// pale, ple -> true
// pales, pale -> true
// pale, bale -> true
// pale, bake -> false
// Hints:#23, #97, #130
//
// time complexity: O(n); where n is the len of shortest string
pub fn one_away(s: &str, edited: &str) -> bool {
    if s.len() == edited.len() {
        one_edit_replace(s, edited)
    } else if s.len() + 1 == edited.len() {
        one_edit_insert(s, edited)
    } else if s.len() - 1 == edited.len() {
        one_edit_insert(edited, s)
    } else {
        false
    }
}

fn one_edit_replace(s1: &str, s2: &str) -> bool {
    let mut found_diff = false;
    for (ch, edit_ch) in s1.chars().zip(s2.chars()) {
        if ch != edit_ch {
            if found_diff {
                return false;
            }
            found_diff = true;
        }
    }
    true
}

fn one_edit_insert(s1: &str, s2: &str) -> bool {
    let mut idx1 = 0;
    let mut idx2 = 0;

    while idx2 < s2.len() && idx1 < s1.len() {
        if s1.chars().nth(idx1) != s2.chars().nth(idx2) {
            if idx1 != idx2 {
                return false;
            }
            idx2 += 1;
        } else {
            idx1 += 1;
            idx2 += 1;
        }
    }

    true
}

pub fn one_away_monolith(first: &str, second: &str) -> bool {
    if first.len().abs_diff(second.len()) > 1 {
        return false;
    }

    let s1 = if first.len() < second.len() { first } else { second };
    let s2 = if first.len() < second.len() { second } else { first };

    let mut idx1 = 0;
    let mut idx2 = 0;
    let mut found_diff = false;

    while idx2 < s2.len() && idx1 < s1.len() {
        if s1.chars().nth(idx1) != s2.chars().nth(idx2) {
            if found_diff { return false; }
            found_diff = true;

            if s1.len() == s2.len() {
                idx1 += 1;
            }
        } else {
            idx1 += 1;
        }
        idx2 += 1;
    }

    true
}

#[test]
fn test_one_away() {
    assert!(one_away("pale", "pale"));
    assert!(one_away("pale", "ple"));
    assert!(one_away("pales", "pale"));
    assert!(one_away("pale", "bale"));
    assert!(!one_away("pale", "bake"));

    assert!(one_away_monolith("pale", "pale"));
    assert!(one_away_monolith("pale", "ple"));
    assert!(one_away_monolith("pales", "pale"));
    assert!(one_away_monolith("pale", "bale"));
    assert!(!one_away_monolith("pale", "bake"));
}

// 1.6 String Compression: Implement a method to perform basic string compression using the counts
// of repeated characters. For example, the string aabcccccaaa would become a2bc5a3. If the
// "compressed" string would not become smaller than the original string, your method should return
// the original string. You can assume the string has only uppercase and lowercase letters (a - z).
// Hints: #92, #110
pub fn str_compression(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_char = None;
    let mut char_count = 2;
    let mut idx_first = 0;

    for ch in s.chars() {
        if last_char == Some(ch) {
            if char_count == 2 {
                out.push('2');
                idx_first = out.len() - 1;
            } else {
                out.truncate(idx_first);
                out.push_str(&char_count.to_string());
            }

            char_count += 1;
        } else {
            out.push(ch);
            char_count = 2;
        }
        last_char = Some(ch);
    }

    out
}

pub fn str_compression_vec(chars: &mut Vec<char>) -> i32 {
    let (mut next_ch, mut start_rep, n) = (0, 0, chars.len());

    for end_rep in 1..=n {
        if end_rep == n || chars[end_rep] != chars[start_rep] {
            chars[next_ch] = chars[start_rep];
            next_ch += 1;
            let diff = end_rep - start_rep;
            if diff > 1 {
                for c in diff.to_string().chars() {
                    chars[next_ch] = c;
                    next_ch += 1;
                }
            }
            start_rep = end_rep;
        }
    }

    chars.truncate(next_ch);

    next_ch as i32
}

pub fn str_compr_bad(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut count_consecutive = 0;

    for i in 0..s.len() {
        count_consecutive += 1;

        if i + 1 >= s.len() || s.chars().nth(i) != s.chars().nth(i + 1) {
            out.push(s.chars().nth(i).unwrap());
            out.push_str(&count_consecutive.to_string());
            count_consecutive = 0;
        }
    }

    if out.len() < s.len() { out } else { s.to_owned() }
}

pub fn str_compr_w_count(s: &str) -> String {
    let compr_len = count_compression(s);

    if compr_len >= s.len() { return s.to_owned(); }

    let mut out = String::with_capacity(compr_len);
    let mut count_consecutive = 0;

    for i in 0..s.len() {
        count_consecutive += 1;

        if i + 1 >= s.len() || s.chars().nth(i) != s.chars().nth(i + 1) {
            out.push(s.chars().nth(i).unwrap());
            out.push_str(&count_consecutive.to_string());
            count_consecutive = 0;
        }
    }

    out
}

fn count_compression(s: &str) -> usize {
    let mut compr_len = 0;
    let mut count_consecutive = 0;

    for i in 0..s.len() {
        count_consecutive += 1;

        if i + 1 >= s.len() || s.chars().nth(i) != s.chars().nth(i + 1) {
            compr_len += 1; // s.chars().nth(i).unwrap().len();
            compr_len += count_consecutive.to_string().len();
            count_consecutive = 0;
        }
    }

    compr_len
}

#[test]
fn test_str_compr() {
    assert_eq!(str_compression("aaaaabcccccdeeef"), "a5bc5de3f");
    assert_eq!(str_compression("aaabbbcccddd"), "a3b3c3d3");
    assert_eq!(str_compression("aabbbbcdddd"), "a2b4cd4");
    assert_eq!(str_compression("aabcccccaaa"), "a2bc5a3");
    assert_eq!(str_compression("abccddefg"), "abc2d2efg");
    assert_eq!(str_compression("abcdefg"), "abcdefg");

    let mut s = vec!['a','a','b','b','c','c','c'];
    str_compression_vec(&mut s);
    assert_eq!(s, vec!['a','2','b','2','c','3']);

    let mut s = vec!['a','a','a'];
    str_compression_vec(&mut s);
    assert_eq!(s, vec!['a','3']);

    assert_eq!(str_compr_bad("aaaaabcccccdeeef"), "a5b1c5d1e3f1");
    assert_eq!(str_compr_bad("aaabbbcccddd"), "a3b3c3d3");
    assert_eq!(str_compr_bad("aabbbbcdddd"), "a2b4c1d4");
    assert_eq!(str_compr_bad("aabcccccaaa"), "a2b1c5a3");
    assert_eq!(str_compr_bad("abccddefg"), "abccddefg");
    assert_eq!(str_compr_bad("abcdefg"), "abcdefg");

    assert_eq!(str_compr_w_count("aaaaabcccccdeeef"), "a5b1c5d1e3f1");
    assert_eq!(str_compr_w_count("aaabbbcccddd"), "a3b3c3d3");
    assert_eq!(str_compr_w_count("aabbbbcdddd"), "a2b4c1d4");
    assert_eq!(str_compr_w_count("aabcccccaaa"), "a2b1c5a3");
    assert_eq!(str_compr_w_count("abccddefg"), "abccddefg");
    assert_eq!(str_compr_w_count("abcdefg"), "abcdefg");
}

// 1.7 Rotate Matrix: Given an image represented by an NxN matrix, where each pixel in the image is 4
// bytes, write a method to rotate the image by 90 degrees. Can you do this in place?
// Hints: #51, #100
// 
// for i = 0 to n:
//   temp = top[i]
//   top[i] = left[i]
//   left[i] = bottom[i]
//   bottom[i] = right[i]
//   right[i] = temp
//
//   xxxy    x goes to y
//   w  y    y -> z
//   w  y    z -> w
//   wzzz    w -> x
//
// time complexity: O(n²)
pub fn rotate_matrix(m: &mut Vec<Vec<u32>>) {
    let n = m.len();
    let mut layer = 0;
    while layer < n / 2 {
        let first = layer;
        let last = n - 1 - layer;
        let mut i = first;
        while i < last {
            let offset = i - first;
            // save top
            let top = m[first][i];
            // left -> top
            m[first][i] = m[last - offset][first];
            // bottom -> left
            m[last - offset][first] = m[last][last - offset];
            // right -> bottom
            m[last][last - offset] = m[i][last];
            // top -> right
            m[i][last] = top;
            i += 1;
        }

        layer += 1;
    }
}

#[test]
fn test_rotate_matrix() {
    let mut matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    rotate_matrix(&mut matrix);
    assert_eq!(matrix, vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]]);

    let mut matrix = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
    rotate_matrix(&mut matrix);
    assert_eq!(matrix, vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]]);
}
