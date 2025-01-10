mod single;
use std::thread::{self, JoinHandle};

fn main() {
    let instance = single::MyStruct::new();
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..1000{
        let handle = thread::spawn(move || {
            instance.lock().unwrap().add();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }

    let instance = single::MyStruct::new();

    println!("{:?}", instance.lock().unwrap());

}
