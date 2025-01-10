use std::sync::{Arc, Mutex};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct MyStruct {
    num: u32,
}

static INSTANCE: OnceLock<Arc<Mutex<MyStruct>>> = OnceLock::new();

impl MyStruct {
    pub fn new() -> &'static Arc<Mutex<MyStruct>> {
        INSTANCE.get_or_init(|| Arc::new(Mutex::new(MyStruct { num: 0 })))
    }

    pub fn add(&mut self) {
        self.num += 1;
    }
}
