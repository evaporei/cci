const assert = require('assert')

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

const BASE_CAP = 500;

class HashMap {
  constructor() {
    this.capacity = BASE_CAP;
    this.buckets = new Array(this.capacity).fill(null);
  }

  get(key) {
    let bucket = this.hash(key) % this.buckets.length;
    let value = this.buckets[bucket];
    return value;
  }

  set(key, value) {
    let bucket = this.hash(key) % this.buckets.length;
    this.buckets[bucket] = value;
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
