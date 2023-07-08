fn adler32(text: &str) -> u32 {
  let mut s1 = 1;
  let mut s2 = 0;

  for c in text.chars() {
    let n = u32::from(c);
    s1 = (s1 + n) % 65521;
    s2 = (s2 + s1) % 65521;
  }

  return (s2 << 16) | s1;
}

const BASE_CAP: usize = 500;

pub struct HashMap<V> {
    entries: [Option<V>; BASE_CAP],
}

impl<V> HashMap<V> {
    pub fn new() -> Self {
        Self { entries: std::array::from_fn(|_| None) }
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        let bucket = adler32(key) as usize % BASE_CAP;
        self.entries[bucket].as_ref()
    }

    pub fn set(&mut self, key: &str, value: V) {
        let bucket = adler32(key) as usize % BASE_CAP;
        self.entries[bucket] = Some(value);
    }
}

#[test]
fn test_map() {
    let mut map = HashMap::new();
    assert_eq!(map.get("2"), None);

    map.set("2", 50);
    assert_eq!(map.get("2"), Some(&50));

    map.set("2", 60);
    assert_eq!(map.get("2"), Some(&60));
}
