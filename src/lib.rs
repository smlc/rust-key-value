use std::collections::HashMap;

#[warn(unused_variables)]

pub struct KvStore {
    dict: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore{ dict: HashMap::new()}
    }
    pub fn set(&mut self, _key: String, _value: String) {
        self.dict.insert(_key, _value);
    }
    
    pub fn get(&self, _key: String) -> Option<String> {
        return self.dict.get(&_key).cloned();
    }

    pub fn remove(&mut self, _key: String){
        self.dict.remove(&_key);
    }
}
