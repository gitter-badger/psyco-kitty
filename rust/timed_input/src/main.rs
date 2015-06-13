
use std::thread;
use std::io;
use std::sync::{Mutex, Arc};
use std::string::String;
use std::mem;


///this time, the goal of the program is to get an input with timeout using multithreading.

fn main() {
    println!("{:?}",read() );
}


fn read() -> String {
    
    let prize = Arc::new(Mutex::new(0));
    let input= Arc::new(Mutex::new(String::new()));



    //first thread

    let prize1 = prize.clone();
    let input1 = input.clone(); 
    let input_t = thread::spawn(move || {
        let mut input1 = input1.lock().unwrap();
        io::stdin().read_line(&mut *input1)
        .ok()
        .expect("Failed to read line");
        let mut prize1 = prize1.lock().unwrap();
        if *prize1 == 0 {
            *prize1 = 1;
        }

    });

    //second thread

    let prize2 = prize.clone();
    let timeout_t = thread::spawn(move || {
        thread::sleep_ms(6000);
        let mut prize2 = prize2.lock().unwrap();
        if *prize2 == 0 {
            *prize2 = 2;
        }

    });





    timeout_t.join().unwrap();
    let prize = *prize.lock().unwrap();
    let mut result = input.lock().unwrap();
    if prize == 1 {
        return mem::replace(&mut *result, String::new());
    }
    "Time has passed".to_string()

}
