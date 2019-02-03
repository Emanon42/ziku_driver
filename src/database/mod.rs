use std::collections::HashMap;

pub struct KVdb {
    pub database: HashMap<String, String>,
}

pub enum CRUD_Result <'a> {
    KeyNotExist, 
    ValueFinded(&'a String),
    ValueInserted, 
    ValueOverwritted(String),
    EntryDeleted(String, String),
    EntriesFinded(Vec<(&'a String, &'a String)>)

}


impl KVdb {
    pub fn new() -> KVdb {
        let mut db: HashMap<String, String> = HashMap::new();
        KVdb{
            database: db,
        }
    }

    // wrapper of raw api
    pub fn get(&self, k: &str) -> CRUD_Result {
        let result = match self.database.get(k) {
            None => CRUD_Result::KeyNotExist,
            Some(s) => CRUD_Result::ValueFinded(s),
        };
        result
    }

    pub fn set(&mut self, k: &str, v: &str) -> CRUD_Result {
        let result = match self.database.insert(String::from(k), String::from(v)) {
            None => CRUD_Result::ValueInserted,
            Some(s) => CRUD_Result::ValueOverwritted(s),
        };
        result
    }

    pub fn delete(&mut self, k: &str) -> CRUD_Result {
        let result = match self.database.remove_entry(k) {
            None => CRUD_Result::KeyNotExist,
            Some((k, v)) => CRUD_Result::EntryDeleted(k, v),
        };
        result
    }

    //now just simple scan, consider to add Scanner to allow customing scanning rule
    pub fn scan(&self, word: &str) -> CRUD_Result {
        let mut vec = Vec::new();
        for (k, v) in &self.database {
            if k.contains(word) {
                vec.push((k, v));
            }
        };
        let result = match vec.is_empty() {
            true => CRUD_Result::KeyNotExist,
            false => CRUD_Result::EntriesFinded(vec),
        };
        result
    }
}

#[test]
fn test_CRUD_incremental(){
    // TODO: implement unit test for database
}