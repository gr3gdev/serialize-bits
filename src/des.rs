use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
    net::SocketAddr,
};

/// # DeserializerData
///
/// Trait for convert data to a Struct.
/// 
/// The library already implements the trait for :
/// - u8, u16, u32, u64, u128, usize
/// - i8, i16, i32, i64, i128, isize
/// - char
/// - bool
/// - String
/// - Option<T>
/// - SocketAddr
/// - Vec<T>, VecDeque<T>, LinkedList<T>
/// - HashSet<T>, BTreeSet<T>
/// - BinaryHeap<T>
/// - HashMap<K, V>, BTreeMap<K, V>
pub trait DeserializerData {
    /// Convert bits (Vec<u8>) into Struct with the next index for convert another Struct.
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized;
}

fn sub(struct_name: &str, data: &Vec<u8>, index: usize, size: usize) -> Vec<u8> {
    if index + size <= data.len() {
        data[index..index + size].to_vec()
    } else {
        panic!("Error when deserialize {struct_name}, index too large {} > {}, (index={index}, size={size})", index + size, data.len())
    }
}

#[cfg(target_pointer_width = "64")]
impl DeserializerData for usize {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            usize::from_ne_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
                data[index + 4],
                data[index + 5],
                data[index + 6],
                data[index + 7],
            ]),
            index + 8,
        )
    }
}

#[cfg(target_pointer_width = "32")]
impl DeserializerData for usize {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            usize::from_ne_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
            ]),
            index + 4,
        )
    }
}

impl DeserializerData for u8 {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (u8::from_ne_bytes([data[index]]), index + 1)
    }
}

impl DeserializerData for u16 {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            u16::from_ne_bytes([data[index], data[index + 1]]),
            index + 2,
        )
    }
}

impl DeserializerData for u32 {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            u32::from_ne_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
            ]),
            index + 4,
        )
    }
}

impl DeserializerData for u64 {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            u64::from_ne_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
                data[index + 4],
                data[index + 5],
                data[index + 6],
                data[index + 7],
            ]),
            index + 8,
        )
    }
}

impl DeserializerData for u128 {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            u128::from_ne_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
                data[index + 4],
                data[index + 5],
                data[index + 6],
                data[index + 7],
                data[index + 8],
                data[index + 9],
                data[index + 10],
                data[index + 11],
                data[index + 12],
                data[index + 13],
                data[index + 14],
                data[index + 15],
            ]),
            index + 16,
        )
    }
}

#[cfg(target_pointer_width = "64")]
impl DeserializerData for isize {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            isize::from_ne_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
                data[index + 4],
                data[index + 5],
                data[index + 6],
                data[index + 7],
            ]),
            index + 8,
        )
    }
}

#[cfg(target_pointer_width = "32")]
impl DeserializerData for isize {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            isize::from_ne_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
            ]),
            index + 4,
        )
    }
}

impl DeserializerData for i8 {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (i8::from_ne_bytes([data[index]]), index + 1)
    }
}

impl DeserializerData for i16 {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            i16::from_ne_bytes([data[index], data[index + 1]]),
            index + 2,
        )
    }
}

impl DeserializerData for i32 {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            i32::from_ne_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
            ]),
            index + 4,
        )
    }
}

impl DeserializerData for i64 {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            i64::from_ne_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
                data[index + 4],
                data[index + 5],
                data[index + 6],
                data[index + 7],
            ]),
            index + 8,
        )
    }
}

impl DeserializerData for i128 {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        (
            i128::from_ne_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
                data[index + 4],
                data[index + 5],
                data[index + 6],
                data[index + 7],
                data[index + 8],
                data[index + 9],
                data[index + 10],
                data[index + 11],
                data[index + 12],
                data[index + 13],
                data[index + 14],
                data[index + 15],
            ]),
            index + 16,
        )
    }
}

impl DeserializerData for bool {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (value, index) = u8::from_data(data, index);
        (value == 1, index)
    }
}

impl DeserializerData for char {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (bit, index) = u8::from_data(data, index);
        (bit as char, index)
    }
}

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

impl<T: DeserializerData> DeserializerData for Option<T> {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (code, index) = u8::from_data(data, index);
        if code == 1 {
            let (value, index) = T::from_data(data, index);
            (Some(value), index)
        } else {
            (None, index)
        }
    }
}

impl<T: DeserializerData> DeserializerData for Vec<T> {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let mut res = Self::new();
        let (size, index) = usize::from_data(data, index);
        let mut list = sub("Vec", data, index, size);
        while !list.is_empty() {
            let (e, e_index) = T::from_data(&list, 0);
            res.push(e);
            list = list[e_index..].to_vec();
        }
        (res, index + size)
    }
}

impl<T: DeserializerData> DeserializerData for VecDeque<T> {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let mut res = Self::new();
        let (size, index) = usize::from_data(data, index);
        let mut list = sub("VecDeque", data, index, size);
        while !list.is_empty() {
            let (e, e_index) = T::from_data(&list, 0);
            res.push_back(e);
            list = list[e_index..].to_vec();
        }
        (res, index + size)
    }
}

impl<T: DeserializerData> DeserializerData for LinkedList<T> {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let mut res = Self::new();
        let (size, index) = usize::from_data(data, index);
        let mut list = sub("LinkedList", data, index, size);
        while !list.is_empty() {
            let (e, e_index) = T::from_data(&list, 0);
            res.push_back(e);
            list = list[e_index..].to_vec();
        }
        (res, index + size)
    }
}

impl<T: DeserializerData> DeserializerData for HashSet<T>
where
    T: Eq,
    T: PartialEq,
    T: Hash,
{
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let mut res = Self::new();
        let (size, index) = usize::from_data(data, index);
        let mut list = sub("HashSet", data, index, size);
        while !list.is_empty() {
            let (e, e_index) = T::from_data(&list, 0);
            res.insert(e);
            list = list[e_index..].to_vec();
        }
        (res, index + size)
    }
}

impl<T: DeserializerData> DeserializerData for BTreeSet<T>
where
    T: Eq,
    T: PartialEq,
    T: Hash,
    T: Ord,
{
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let mut res = Self::new();
        let (size, index) = usize::from_data(data, index);
        let mut list = sub("BTreeSet", data, index, size);
        while !list.is_empty() {
            let (e, e_index) = T::from_data(&list, 0);
            res.insert(e);
            list = list[e_index..].to_vec();
        }
        (res, index + size)
    }
}

impl<T: DeserializerData> DeserializerData for BinaryHeap<T>
where
    T: Eq,
    T: PartialEq,
    T: Hash,
    T: Ord,
{
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let mut res = Self::new();
        let (size, index) = usize::from_data(data, index);
        let mut list = sub("BinaryHeap", data, index, size);
        while !list.is_empty() {
            let (e, e_index) = T::from_data(&list, 0);
            res.push(e);
            list = list[e_index..].to_vec();
        }
        (res, index + size)
    }
}

impl<K: DeserializerData, V: DeserializerData> DeserializerData for HashMap<K, V>
where
    K: Eq,
    K: PartialEq,
    K: Hash,
{
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let mut res = Self::new();
        let (size, index) = usize::from_data(data, index);
        let mut list = sub("HashMap", data, index, size);
        while !list.is_empty() {
            let (key, e_index) = K::from_data(&list, 0);
            let (value, e_index) = V::from_data(&list, e_index);
            res.insert(key, value);
            list = list[e_index..].to_vec();
        }
        (res, index + size)
    }
}

impl<K: DeserializerData, V: DeserializerData> DeserializerData for BTreeMap<K, V>
where
    K: Eq,
    K: PartialEq,
    K: Hash,
    K: Ord,
{
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let mut res = Self::new();
        let (size, index) = usize::from_data(data, index);
        let mut list = sub("BTreeMap", data, index, size);
        while !list.is_empty() {
            let (key, e_index) = K::from_data(&list, 0);
            let (value, e_index) = V::from_data(&list, e_index);
            res.insert(key, value);
            list = list[e_index..].to_vec();
        }
        (res, index + size)
    }
}

impl DeserializerData for SocketAddr {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (value, index) = String::from_data(data, index);
        let addr = value.parse().unwrap();
        (addr, index)
    }
}
