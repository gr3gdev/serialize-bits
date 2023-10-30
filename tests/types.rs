use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    net::SocketAddr,
};

use serialize_bits::{des::DeserializerData, ser::SerializerData};

#[test]
pub fn test_usize() {
    let size = 2034 as usize;
    let data = size.to_data();
    assert_eq!(vec![242, 7, 0, 0, 0, 0, 0, 0], data);
    assert_eq!((size, 8), usize::from_data(&data, 0));
}

#[test]
pub fn test_u8() {
    let size = 234 as u8;
    let data = size.to_data();
    assert_eq!(vec![234], data);
    assert_eq!((size, 1), u8::from_data(&data, 0));
}

#[test]
pub fn test_u16() {
    let size = 555 as u16;
    let data = size.to_data();
    assert_eq!(vec![43, 2], data);
    assert_eq!((size, 2), u16::from_data(&data, 0));
}

#[test]
pub fn test_u32() {
    let size = 50505 as u32;
    let data = size.to_data();
    assert_eq!(vec![73, 197, 0, 0], data);
    assert_eq!((size, 4), u32::from_data(&data, 0));
}

#[test]
pub fn test_u64() {
    let size = 980765 as u64;
    let data = size.to_data();
    assert_eq!(vec![29, 247, 14, 0, 0, 0, 0, 0], data);
    assert_eq!((size, 8), u64::from_data(&data, 0));
}

#[test]
pub fn test_u128() {
    let size = 1234567890 as u128;
    let data = size.to_data();
    assert_eq!(
        vec![210, 2, 150, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        data
    );
    assert_eq!((size, 16), u128::from_data(&data, 0));
}

#[test]
pub fn test_isize() {
    let size = 2034 as isize;
    let data = size.to_data();
    assert_eq!(vec![242, 7, 0, 0, 0, 0, 0, 0], data);
    assert_eq!((size, 8), isize::from_data(&data, 0));
}

#[test]
pub fn test_i8() {
    let size = 120 as i8;
    let data = size.to_data();
    assert_eq!(vec![120], data);
    assert_eq!((size, 1), i8::from_data(&data, 0));
}

#[test]
pub fn test_i16() {
    let size = 555 as i16;
    let data = size.to_data();
    assert_eq!(vec![43, 2], data);
    assert_eq!((size, 2), i16::from_data(&data, 0));
}

#[test]
pub fn test_i32() {
    let size = 50505 as i32;
    let data = size.to_data();
    assert_eq!(vec![73, 197, 0, 0], data);
    assert_eq!((size, 4), i32::from_data(&data, 0));
}

#[test]
pub fn test_i64() {
    let size = 980765 as i64;
    let data = size.to_data();
    assert_eq!(vec![29, 247, 14, 0, 0, 0, 0, 0], data);
    assert_eq!((size, 8), i64::from_data(&data, 0));
}

#[test]
pub fn test_i128() {
    let size = 1234567890 as i128;
    let data = size.to_data();
    assert_eq!(
        vec![210, 2, 150, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        data
    );
    assert_eq!((size, 16), i128::from_data(&data, 0));
}

#[test]
pub fn test_bool() {
    let value = true;
    let data = value.to_data();
    assert_eq!(vec![1], data);
    assert_eq!((value, 1), bool::from_data(&data, 0));
}

#[test]
pub fn test_string() {
    let value = String::from("This is a test value");
    let data = value.to_data();
    assert_eq!(
        vec![
            20, 0, 0, 0, 0, 0, 0, 0, 84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115,
            116, 32, 118, 97, 108, 117, 101
        ],
        data
    );
    assert_eq!((value, 28), String::from_data(&data, 0));
}

#[test]
pub fn test_option() {
    let value = Some(String::from("This is optional"));
    let data = value.to_data();
    assert_eq!(
        vec![
            1, 16, 0, 0, 0, 0, 0, 0, 0, 84, 104, 105, 115, 32, 105, 115, 32, 111, 112, 116, 105,
            111, 110, 97, 108
        ],
        data
    );
    assert_eq!((value, 25), Option::from_data(&data, 0));
}

#[test]
pub fn test_vec() {
    let value = vec![String::from("Value 1"), String::from("Value 2")];
    let data = value.to_data();
    let expected = vec![
        30, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 49, 7, 0, 0, 0,
        0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 50,
    ];
    assert_eq!(expected, data);
    assert_eq!((value, expected.len()), Vec::from_data(&data, 0));
}

#[test]
pub fn test_vec_deque() {
    let mut value = VecDeque::new();
    value.push_front(String::from("Value 1"));
    value.push_front(String::from("Value 2"));
    let data = value.to_data();
    assert_eq!(
        vec![
            30, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 50, 7, 0,
            0, 0, 0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 49
        ],
        data
    );
    assert_eq!((value, 38), VecDeque::from_data(&data, 0));
}

#[test]
pub fn test_linkedlist() {
    let mut value = LinkedList::new();
    value.push_front(String::from("Value 1"));
    value.push_front(String::from("Value 2"));
    let data = value.to_data();
    assert_eq!(
        vec![
            30, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 50, 7, 0,
            0, 0, 0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 49
        ],
        data
    );
    assert_eq!((value, 38), LinkedList::from_data(&data, 0));
}

#[test]
pub fn test_hashset() {
    let mut value = HashSet::new();
    value.insert(String::from("Value 1"));
    let data = value.to_data();
    assert_eq!(
        vec![15, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 49],
        data
    );
    assert_eq!((value, 23), HashSet::from_data(&data, 0));
}

#[test]
pub fn test_btreeset() {
    let mut value = BTreeSet::new();
    value.insert(String::from("Value 1"));
    value.insert(String::from("Value 2"));
    let data = value.to_data();
    assert_eq!(
        vec![
            30, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 49, 7, 0,
            0, 0, 0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 50
        ],
        data
    );
    assert_eq!((value, 38), BTreeSet::from_data(&data, 0));
}

#[test]
pub fn test_binaryheap() {
    let mut value = BinaryHeap::new();
    value.push(String::from("Value 1"));
    value.push(String::from("Value 2"));
    let data = value.to_data();
    assert_eq!(
        vec![
            30, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 50, 7, 0,
            0, 0, 0, 0, 0, 0, 86, 97, 108, 117, 101, 32, 49
        ],
        data
    );
    let (mut res, size): (BinaryHeap<String>, usize) = BinaryHeap::from_data(&data, 0);
    assert_eq!(2, res.len());
    assert_eq!(String::from("Value 2"), res.pop().unwrap());
    assert_eq!(String::from("Value 1"), res.pop().unwrap());
    assert_eq!(38, size);
}

#[test]
pub fn test_hashmap() {
    let mut value = HashMap::new();
    value.insert(String::from("KEY1"), true);
    let data = value.to_data();
    assert_eq!(
        vec![13, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 75, 69, 89, 49, 1],
        data
    );
    assert_eq!((value, 21), HashMap::from_data(&data, 0));
}

#[test]
pub fn test_btreemap() {
    let mut value = BTreeMap::new();
    value.insert(String::from("KEY1"), 255);
    value.insert(String::from("KEY2"), 896);
    value.insert(String::from("KEY3"), 120394);
    let data = value.to_data();
    assert_eq!(
        vec![
            48, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 75, 69, 89, 49, 255, 0, 0, 0, 4, 0, 0,
            0, 0, 0, 0, 0, 75, 69, 89, 50, 128, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 75, 69, 89, 51,
            74, 214, 1, 0
        ],
        data
    );
    assert_eq!((value, 56), BTreeMap::from_data(&data, 0));
}

#[test]
pub fn test_socketaddr() {
    let value: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let data = value.to_data();
    assert_eq!(
        vec![14, 0, 0, 0, 0, 0, 0, 0, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 51, 48, 48, 48],
        data
    );
    assert_eq!((value, 22), SocketAddr::from_data(&data, 0));
}
