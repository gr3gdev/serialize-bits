[![Build & Tests](https://github.com/gr3gdev/serialize-bits/actions/workflows/build.yml/badge.svg)](https://github.com/gr3gdev/serialize-bits/actions/workflows/build.yml)
[![Rust Docs](https://img.shields.io/badge/docs.rs-serialize_bits-green)](https://docs.rs/serialize_bits/)

# serialize-bits
Rust serializer/deserializer : Struct to bits, bits to Struct.

The library already implements the traits for :
- u8, u16, u32, u64, u128, usize
- i8, i16, i32, i64, i128, isize
- char
- bool
- String
- Option<T>
- SocketAddr
- Vec<T>, VecDeque<T>, LinkedList<T>
- HashSet<T>, BTreeSet<T>
- BinaryHeap<T>
- HashMap<K, V>, BTreeMap<K, V>

## Serialization

Implement the SerializerData trait.

Example :

```rust
impl SerializerData for String {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.append(&mut self.len().to_data());
        res.append(&mut self.as_bytes().to_vec());
        res
    }
}
```

## Deserialization

Implement the DeserializerData trait.

Example :

```rust
impl DeserializerData for String {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (size, index) = usize::from_data(data, index);
        let list = sub("String", data, index, size);
        (String::from_utf8(list).unwrap(), index + size)
    }
}
```
