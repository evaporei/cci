const assert = require('assert')

class LinkedList {
  constructor(data) {
    this.data = data;
    this.next = null;
  }
}

function adler32(text) {
  let s1 = 1;
  let s2 = 0;

  for (let i = 0; i < text.length; i++) {
    let n = text.charCodeAt(i);
    s1 = (s1 + n) % 65521;
    s2 = (s2 + s1) % 65521;
  }

  return (s2 << 16) | s1;
}

const dbg = (label, data) => {
  if (data == null) {
    data = label;
    label = null;
  }

  if (label == null) {
    console.log(data);
  } else {
    console.log(label, data);
  }

  return data;
}

const BASE_CAP = 500;

class HashMap {
  constructor() {
    this.capacity = BASE_CAP;
    this.buckets = new Array(this.capacity).fill(null);
  }

  get(key) {
    let bucket = this.hash(key) % this.buckets.length;
    let list = this.buckets[bucket];
    if (list) {
      let node = list;
      while (node) {
        if (String(node.data.key) === String(key)) {
          return node.data.value;
        }
        node = node.next;
      }
    }
    return null;
  }

  set(key, value) {
    let bucket = this.hash(key) % this.buckets.length;
    if (this.buckets[bucket]) {
      let node = this.buckets[bucket];
      while (node.next) {
        node = node.next;
      }
      node.next = new LinkedList({ key, value });
    } else {
      this.buckets[bucket] = new LinkedList({ key, value });
    }
  }

  hash(key) {
    return adler32(String(key))
  }
}

let map = new HashMap()

assert.deepEqual(map.get(2), null)

map.set(2, 50)

assert.deepEqual(map.get(2), 50)

map.set(2, 60)

assert.deepEqual(map.get(2), 60)
