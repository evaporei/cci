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

#[derive(Debug)]
struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T> LinkedList<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: None,
        }
    }
}

const BASE_CAP: usize = 500;

#[derive(Debug)]
pub struct HashMap<'a, V> {
    entries: [Option<Box<LinkedList<(&'a str, V)>>>; BASE_CAP],
}

impl<'a, V> HashMap<'a, V> {
    pub fn new() -> Self {
        Self { entries: std::array::from_fn(|_| None) }
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        let bucket = adler32(key) as usize % BASE_CAP;
        let mut list = self.entries[bucket].as_ref();
        while let Some(node) = list {
            if node.data.0 == key {
                return Some(&node.data.1);
            } else {
                list = node.next.as_ref();
            }
        }
        None
    }

    pub fn set(&mut self, key: &'a str, value: V) {
        let bucket = adler32(key) as usize % BASE_CAP;
        let mut list = self.entries[bucket].as_mut();
        if list.is_some() {
            while let Some(node) = list {
                list = node.next.as_mut();
            }
            list.replace(&mut Box::new(LinkedList::new((key, value))));
        } else {
            self.entries[bucket] = Some(Box::new(LinkedList::new((key, value))));
        }
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

    dbg!(map);
}
