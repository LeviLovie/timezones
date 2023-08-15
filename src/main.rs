#[allow(dead_code)]

use std::io;
use chrono::Local;

struct User {
    name: String,
    tz_offset: i32,
}

fn main() {
    let mut users: Vec<User> = vec![];
    users.push(User{name: "Loc".to_string(), tz_offset: 0});

    let mut command: String = "".to_string();
    let mut error: String = "".to_string();
    let mut range_start: i32 = 0;
    let mut range_end: i32 = 0;
    loop {
        let time = Local::now();
        print!("\x1b[2J\x1b[1;1H");
        println!("Your current time is \x1b[36m\x1b[1m{}\x1b[0m", time);

        if command != "" {
            let command_splited: Vec<&str> = command.split(" ").collect();
            if command == "exit" {
                print!("\x1b[2J\x1b[1;1H");
                println!("Thank you for using this appliation!");
                std::process::exit(0);
            } else if command_splited[0] == "add" {
                if command_splited.len() > 2 && command_splited.len() < 4 {
                    let name: String = command_splited[1].to_string();
                    let timezome_offset: Result<i32, std::num::ParseIntError> = command_splited[2].parse();
                    match timezome_offset {
                        Ok(timezone_offset_number) => {
                            users.push(User{name: name, tz_offset: timezone_offset_number})
                        }
                        Err(parse_error) => {
                            error = format!("\x1b[31mConverting second arg (UTC timezone offset) to integer end unsuccessfully\x1b[0m: \x1b[1m{}\x1b[0m", parse_error);
                        }
                    }
                } else {
                    error = format!("\x1b[31mUnexpected amount of args\x1b[0m: \x1b[1m{}\x1b[0m", command);
                }
            } else if command_splited[0] == "range" {
                if command_splited.len() > 2 && command_splited.len() < 4 {
                    let start: Result<i32, std::num::ParseIntError> = command_splited[1].parse();
                    let end: Result<i32, std::num::ParseIntError> = command_splited[2].parse();
                    match start {
                        Ok(start_number) => {
                            match end {
                                Ok(end_number) => {
                                    range_start = start_number;
                                    range_end = end_number;
                                }
                                Err(parse_error) => {
                                    error = format!("\x1b[31mConverting second arg (range_end) to integer end unsuccessfully\x1b[0m: \x1b[1m{} - {}\x1b[0m", command, parse_error);
                                }
                            }
                        }
                        Err(parse_error) => {
                            error = format!("\x1b[31mConverting first arg (range_start) to integer end unsuccessfully\x1b[0m: \x1b[1m{} - {}\x1b[0m", command, parse_error);
                        }
                    }
                } else {
                    error = format!("\x1b[31mUnexpected amount of args\x1b[0m: \x1b[1m{}\x1b[0m", command);
                }
            } else {
                error = format!("\x1b[31mUnknown command\x1b[0m: \x1b[1m{}\x1b[0m", command);
            }
            command = "".to_string();
        }
        println!("{}", error);
        error = "".to_string();

        print!("+");
        for _ in 0..users.len() {
            print!("-----+");
        }
        print!("\n");

        print!("|");
        for user in 0..users.len() {
            print!("\x1b[36m{:<5}\x1b[0m|", users[user].name);
        }
        print!("\n");

        for hour in 0..24 {
            print!("|");
            for user in 0..users.len() {
                let mut offseted_hour = hour + users[user].tz_offset;
                while offseted_hour > 23 {
                    offseted_hour -= 24;
                }
                while offseted_hour < 0 {
                    offseted_hour += 24;
                }
                if hour >= range_start && hour < range_end {
                    print!("\x1b[31m{:>2}\x1b[0m-\x1b[31m{:>2}\x1b[0m|", offseted_hour, offseted_hour + 1);
                } else {
                    if user == 0 {
                        print!("\x1b[36m{:>2}\x1b[0m-\x1b[36m{:>2}\x1b[0m|", offseted_hour, offseted_hour + 1);
                    } else {
                        print!("\x1b[34m{:>2}\x1b[0m-\x1b[34m{:>2}\x1b[0m|", offseted_hour, offseted_hour + 1);
                    }
                }
            }
            print!("\n")
        }
        print!("\n");

        // Usage
        println!("`exit` - close app; `add <name> <UTC timezone offset>` - add user;\n`range <start hour> <end hour>` - set range of available hours");
        command = get_user_input();
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string()
}

// fn set_cursor_pos(y: i32, x: i32) {
//     print!("\x1b[{};{}H", y, x);
// }

// fn get_terminal_y_size() -> i32 {
//     let (_, y) = termion::terminal_size().unwrap();
//     return y as i32
// }
