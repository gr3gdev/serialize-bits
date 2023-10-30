use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    net::SocketAddr,
};

/// # SerializerData
///
/// Trait for convert Struct to data.
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
pub trait SerializerData {
    /// Convert the Struct into bits (Vec<u8>).
    fn to_data(&self) -> Vec<u8>;
}

impl SerializerData for usize {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for u8 {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for u16 {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for u32 {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for u64 {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for u128 {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for isize {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for i8 {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for i16 {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for i32 {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for i64 {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for i128 {
    fn to_data(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl SerializerData for bool {
    fn to_data(&self) -> Vec<u8> {
        let value = if self.clone() { 1 as u8 } else { 0 as u8 };
        value.to_data()
    }
}

impl SerializerData for char {
    fn to_data(&self) -> Vec<u8> {
        let bit = self.clone() as u8;
        bit.to_data()
    }
}

impl SerializerData for String {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.append(&mut self.len().to_data());
        res.append(&mut self.as_bytes().to_vec());
        res
    }
}

impl<T: SerializerData> SerializerData for Option<T> {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        if let Some(value) = self {
            let code = 1 as u8;
            res.append(&mut code.to_data());
            res.append(&mut value.to_data());
        } else {
            let code = 0 as u8;
            res.append(&mut code.to_data());
        }
        res
    }
}

impl<T: SerializerData> SerializerData for Vec<T> {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let data = self.iter().flat_map(|e| e.to_data()).collect::<Vec<u8>>();
        res.append(&mut data.len().to_data());
        res.append(&mut data.clone());
        res
    }
}

impl<T: SerializerData> SerializerData for VecDeque<T> {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let data = self.iter().flat_map(|e| e.to_data()).collect::<Vec<u8>>();
        res.append(&mut data.len().to_data());
        res.append(&mut data.clone());
        res
    }
}

impl<T: SerializerData> SerializerData for LinkedList<T> {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let data = self.iter().flat_map(|e| e.to_data()).collect::<Vec<u8>>();
        res.append(&mut data.len().to_data());
        res.append(&mut data.clone());
        res
    }
}

impl<T: SerializerData> SerializerData for HashSet<T> {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let data = self.iter().flat_map(|e| e.to_data()).collect::<Vec<u8>>();
        res.append(&mut data.len().to_data());
        res.append(&mut data.clone());
        res
    }
}

impl<T: SerializerData> SerializerData for BTreeSet<T> {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let data = self.iter().flat_map(|e| e.to_data()).collect::<Vec<u8>>();
        res.append(&mut data.len().to_data());
        res.append(&mut data.clone());
        res
    }
}

impl<T: SerializerData> SerializerData for BinaryHeap<T> {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let data = self.iter().flat_map(|e| e.to_data()).collect::<Vec<u8>>();
        res.append(&mut data.len().to_data());
        res.append(&mut data.clone());
        res
    }
}

impl<K: SerializerData, V: SerializerData> SerializerData for HashMap<K, V> {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let mut map = self
            .iter()
            .flat_map(|(key, value)| {
                let mut merge = Vec::new();
                merge.append(&mut key.to_data());
                merge.append(&mut value.to_data());
                merge
            })
            .collect::<Vec<u8>>();
        res.append(&mut map.len().to_data());
        res.append(&mut map);
        res
    }
}

impl<K: SerializerData, V: SerializerData> SerializerData for BTreeMap<K, V> {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let mut map = self
            .iter()
            .flat_map(|(key, value)| {
                let mut merge = Vec::new();
                merge.append(&mut key.to_data());
                merge.append(&mut value.to_data());
                merge
            })
            .collect::<Vec<u8>>();
        res.append(&mut map.len().to_data());
        res.append(&mut map);
        res
    }
}

impl SerializerData for SocketAddr {
    fn to_data(&self) -> Vec<u8> {
        self.to_string().to_data()
    }
}
