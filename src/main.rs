use std::{io, thread};
use std::io::{stdout, Write};
use chrono::{self, Local, NaiveDate};
use std::sync::mpsc;

#[derive(Debug)]
struct Log<'a> {  // struct for logging the times spent studying or on a break with the date
    total_time: &'a i64,
    break_time: &'a i64,
    date: NaiveDate,
}

fn main(){
    let (tx,rx) = mpsc::channel();

    
    let mut stud_time: i64 = 0;
    let mut down_time: i64 = 0; // basically break_time but idk why i named it this and i am too
                                // far in the project to change it now

    let mut input = String::new();
    println!("You are:\n1) Studying\n2) On a break");

    print!("Enter the respective number here: "); // if you are studying press 1 and if on a break
                                                  // press 2. this is as per the message being
                                                  // printed above this line
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Cannot read input."); 

    println!(""); // i just wanted a new line without ruining anything else

    let input = input.trim().to_string();

    thread::spawn(move || { // this thread always listens for any input WHILE the clock is running 
        loop {
            let mut input_two = String::new();
            io::stdin().read_line(&mut input_two).unwrap();
            tx.send(input_two).unwrap();
        }
    });

    loop {
        match rx.try_recv() {  // if there is any input then actions are performed here 
            Ok(msg) => {
                if msg.trim() == "q" {
                    println!("\nExiting...");
                    println!("time studied: {}", stud_time);
                    log(&stud_time, &down_time);
                    break;
                }
            }
            Err(_) => {} 
        }
        if input.to_lowercase() == "1" || input.to_lowercase() == "studying".to_string() {
            print!("\rTime Spent Studying: {} hours {} minutes {} seconds       ", stud_time/3600, stud_time/60, stud_time%60);
            io::Write::flush(&mut io::stdout()).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
            stud_time += 1;

        } else if input == "2" || input.to_lowercase() == "break".to_string() {
            print!("\rTime Spent In Break: {} hours {} minutes {} seconds       ", down_time/3600, down_time/60, down_time%60);
            io::Write::flush(&mut stdout()).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
            down_time += 1
        }
    }
}

fn log<'a>(stud_time: &'a i64, down_time: &'a i64) {
    let date_now = Local::now().date_naive();
    let new_log = Log {
        total_time: stud_time,
        break_time: down_time,
        date: date_now,
    };

    println!("{:?}", new_log);
}
