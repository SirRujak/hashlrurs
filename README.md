![Rust](https://github.com/SirRujak/hashlrurs/workflows/Rust/badge.svg)

# hashlrurs

Rust reimplementation of the simple and fast LRU cache by Dominic Tarr found here: https://github.com/dominictarr/hashlru

# example

let HLRU = HashLRU::new(100);

```
HLRU.set(key, value);
let new_value: usize;
match HLRU.get(key) {
  Ok(v) => new_value = v,
  None => {}
}
```
