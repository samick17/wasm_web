use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen(js_name = Store)]
#[derive(Debug)]
pub struct Store {
    items: HashMap::<String, u32>
}

#[wasm_bindgen]
impl Store {
    pub fn add(&mut self, key: String, value: u32) {
        self.items.insert(key, value);
    }
    pub fn remove(&mut self, key: String) {
        self.items.remove(&key);
    }
    pub fn get(&mut self, key: String) -> u32 {
        match self.items.get(&key) {
            Some(value) => *value,
            _ => 0
        }
    }
}

#[wasm_bindgen]
pub fn new_store() -> Store {
    Store{
        items: HashMap::<String, u32>::new(),
    }
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn api1() -> i32 {
    5
}

#[wasm_bindgen]
pub fn custom_main() {
    alert("Hello");
    log("asdf");
    alert("123");
    log("zxcv");
}
