use std::vec;

const CAPACITY: usize = 26;

type KeyIndex = usize;

#[derive(Debug)]
enum Field {
    Occupied(String),
    Empty,
    Tombstone,
}

// impl std::fmt::Display for Field {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Field::Empty => {
//                 write!(f, "Empty")
//             }
//             Field::Tombstone => {
//                 write!(f, "Tombstone")
//             }
//             Field::Occupied(value) => {
//                 write!(f, "Occupied({value})")
//             }
//         }
//     }
// }

struct HashTable {
    fields: Box<[Field]>,
}

impl std::fmt::Debug for HashTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.fields
                .iter()
                .filter(|f| matches!(f, Field::Occupied(_)))
                .map(|f| format!("{:?}", f))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl HashTable {
    fn new() -> Self {
        let mut f: Vec<Field> = vec![];
        for _ in 0..CAPACITY {
            f.push(Field::Empty);
        }

        Self {
            fields: f.into_boxed_slice(),
        }
    }

    fn is_valid_key(key: String) -> bool {
        if key.len() > 10 {
            return false;
        }

        return true;
    }

    fn search(&self, key: String) -> Option<KeyIndex> {
        if !HashTable::is_valid_key(key.clone()) {
            return None;
        }

        let last_letter = key.chars().last().unwrap();
        let mut index = (last_letter as u8 - 97) as usize;
        let starting_index = index;

        loop {
            let field = &self.fields[index];

            match field {
                Field::Empty => {
                    return None;
                }
                Field::Tombstone => {}
                Field::Occupied(value) => {
                    if value == &key {
                        return Some(index);
                    }
                }
            }

            index = (index + 1) % CAPACITY;
            if index == starting_index {
                return None;
            }
        }
    }

    fn insert(&mut self, key: String) {
        if let Some(_) = self.search(key.clone()) {
            return;
        }

        let last_letter = key.chars().last().unwrap();
        let mut index = (last_letter as u8 - 97) as usize;
        let starting_index = index;

        while matches!(self.fields[index as usize], Field::Occupied(_)) {
            index = (index + 1) % CAPACITY;
            if index == starting_index {
                return;
            }
        }

        self.fields[index] = Field::Occupied(key);
    }

    fn delete(&mut self, key: String) {
        match self.search(key) {
            None => {
                return;
            }
            Some(index) => self.fields[index] = Field::Tombstone,
        }
    }
}

fn main() {
    let mut table = HashTable::new();

    println!("{:?}", table);
    println!("{:?}", table.search(String::from("hello")));

    table.insert(String::from("hello"));
    table.insert(String::from("hellp"));
    table.insert(String::from("hollo"));

    println!("{:?}", table);
    println!("{:?}", table.search(String::from("hollo")));
}
