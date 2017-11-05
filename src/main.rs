#[macro_use]
extern crate chan;
extern crate chrono;
extern crate reqwest;
extern crate colored;

use std::io;
use colored::*;
use chrono::Local;
use std::io::prelude::*;

fn main() {

    println!("
 _____         _      _____     _      _____                     _   
| __  |_ _ ___| |_   |   __|___| |_   | __  |___ ___ _ _ ___ ___| |_ 
|    -| | |_ -|  _|  |  |  | -_|  _|  |    -| -_| . | | | -_|_ -|  _|
|__|__|___|___|_|    |_____|___|_|    |__|__|___|_  |___|___|___|_|  
                                                  |_|                 
");

    println!("{}", "Please enter the target address : ".yellow().bold() );

    let mut stdout = io::stdout();
    write!(&mut stdout, "> ").expect("could not trigger prompt");
    stdout.flush().expect("could not wait response");

    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("could not read prompt");
    
    let url: &str = url.trim();

    let mut count = 0;

    let tick = chan::tick_ms(1);
    
    loop {
        count += 1;
        chan_select! {
            tick.recv() => get_call( count, url ),
        }
    }
}

fn get_call( count: i32, url: &str ) {

    let mut response = reqwest::get(url).expect("Failed to send request");
    let date = Local::now();

    let mut buff = String::new();
    response.read_to_string(&mut buff).expect("Failed to read response");

    println!(" - {} : Status {} for request n°{}, Get {}" , date.format("%d-%m-%Y %H:%M:%S").to_string().yellow().bold(),response.status().to_string().green().bold(),count,buff.green().bold() );
}