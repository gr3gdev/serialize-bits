use serialize_bits::{des::DeserializerData, ser::SerializerData};

#[derive(Clone, Debug, PartialEq)]
struct City {
    id: i64,
    name: String,
}

impl SerializerData for City {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.append(&mut self.id.to_data());
        res.append(&mut self.name.to_data());
        res
    }
}

impl DeserializerData for City {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (id, index) = i64::from_data(data, index);
        let (name, index) = String::from_data(data, index);
        (Self { id, name }, index)
    }
}

#[derive(Debug, PartialEq)]
struct Address {
    id: i64,
    street: String,
    postal_code: String,
    city: City,
}

impl SerializerData for Address {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.append(&mut self.id.to_data());
        res.append(&mut self.street.to_data());
        res.append(&mut self.postal_code.to_data());
        res.append(&mut self.city.to_data());
        res
    }
}

impl DeserializerData for Address {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (id, index) = i64::from_data(data, index);
        let (street, index) = String::from_data(data, index);
        let (postal_code, index) = String::from_data(data, index);
        let (city, index) = City::from_data(data, index);
        (
            Self {
                id,
                street,
                postal_code,
                city,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
enum Genre {
    Male,
    Female,
    Custom(String),
}

impl SerializerData for Genre {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        match self {
            Self::Male => {
                let code = 1 as u8;
                res.append(&mut code.to_data());
            }
            Self::Female => {
                let code = 2 as u8;
                res.append(&mut code.to_data());
            }
            Self::Custom(v) => {
                let code = 3 as u8;
                res.append(&mut code.to_data());
                res.append(&mut v.to_data());
            }
        }
        res
    }
}

impl DeserializerData for Genre {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (code, index) = u8::from_data(data, index);
        match code {
            1 => (Self::Male, index),
            2 => (Self::Female, index),
            3 => {
                let (value, index) = String::from_data(data, index);
                (Self::Custom(value), index)
            }
            _ => panic!("Invalid code !"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
    genre: Genre,
    addresses: Vec<Address>,
}

impl SerializerData for Person {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.append(&mut self.id.to_data());
        res.append(&mut self.name.to_data());
        res.append(&mut self.genre.to_data());
        res.append(&mut self.addresses.to_data());
        res
    }
}

impl DeserializerData for Person {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (id, index) = i32::from_data(data, index);
        let (name, index) = String::from_data(data, index);
        let (genre, index) = Genre::from_data(data, index);
        let (addresses, index) = Vec::from_data(data, index);
        (
            Self {
                id,
                name,
                genre,
                addresses,
            },
            index,
        )
    }
}

#[test]
fn test_struct() {
    let city = City {
        id: 44,
        name: String::from("NANTES"),
    };
    let address1 = Address {
        id: 1,
        street: String::from("avenue du Général de Gaulle"),
        postal_code: String::from("44000"),
        city: city.clone(),
    };
    let address2 = Address {
        id: 2,
        street: String::from("rue Jean Jaurès"),
        postal_code: String::from("44000"),
        city: city.clone(),
    };
    let person = Person {
        id: 35,
        name: String::from("Albert"),
        genre: Genre::Custom(String::from("Doctor")),
        addresses: vec![address1, address2],
    };
    let data = person.to_data();
    assert_eq!(
        vec![
            35, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 65, 108, 98, 101, 114, 116, 3, 6, 0, 0, 0, 0, 0,
            0, 0, 68, 111, 99, 116, 111, 114, 147, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 29,
            0, 0, 0, 0, 0, 0, 0, 97, 118, 101, 110, 117, 101, 32, 100, 117, 32, 71, 195, 169, 110,
            195, 169, 114, 97, 108, 32, 100, 101, 32, 71, 97, 117, 108, 108, 101, 5, 0, 0, 0, 0, 0,
            0, 0, 52, 52, 48, 48, 48, 44, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 78, 65, 78,
            84, 69, 83, 2, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 114, 117, 101, 32, 74,
            101, 97, 110, 32, 74, 97, 117, 114, 195, 168, 115, 5, 0, 0, 0, 0, 0, 0, 0, 52, 52, 48,
            48, 48, 44, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 78, 65, 78, 84, 69, 83
        ],
        data
    );
    let des = Person::from_data(&data, 0);
    assert_eq!((person, 188), des);
}
