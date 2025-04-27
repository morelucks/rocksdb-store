use rocksdb::{DB, Options};

fn main() {
    // Configure DB
    let path = "_demo_rocksdb";
    let mut opts = Options::default();
    opts.create_if_missing(true);

    // Open DB
    let db = DB::open(&opts, path).expect("failed to open db");

    // Write a key-value pair
    db.put(b"mykey", b"myvalue").expect("failed to write");

    // Read the value
    match db.get(b"mykey") {
        Ok(Some(value)) => println!("Retrieved value: {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("Value not found"),
        Err(e) => println!("Error reading value: {}", e),
    }

    // Delete the key
    db.delete(b"mykey").expect("failed to delete key");
}





// use std::rc::Rc;
// use std::sync::Mutex;
// use std::thread;

// fn main() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }