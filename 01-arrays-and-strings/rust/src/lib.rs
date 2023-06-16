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

#[test]
fn test_is_unique() {
    assert!(!is_unique("aabb"));
    assert!(is_unique("asdf"));
}
