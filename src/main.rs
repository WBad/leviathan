use read_input::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
enum ChoiceError {
    InvalidChoice(String),
}

#[derive(Debug)]
enum MainMenuChoice {
    ServerTime,
    XbtInfo,
    OpenOrders,
    Quit,
}

impl FromStr for MainMenuChoice {
    type Err = ChoiceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choice = match s.trim().to_ascii_uppercase().chars().next() {
            Some('1') => MainMenuChoice::ServerTime,
            Some('2') => MainMenuChoice::XbtInfo,
            Some('3') => MainMenuChoice::OpenOrders,
            Some('Q') => MainMenuChoice::Quit,
            _ => return Err(ChoiceError::InvalidChoice(s.to_string())),
        };
        Ok(choice)
    }
}

fn main_menu() -> MainMenuChoice {
    input::<MainMenuChoice>()
        .repeat_msg(format!(
            "What would you like to do?\n[1] {op_1}\n[2] {op_2}\n[3] {op_3}\n[q] {quit}\nInput: ",
            op_1 = "Retrieve server time",
            op_2 = "Retrieve XBT/USD information",
            op_3 = "Retrieve my account's open orders",
            quit = "Quit",
        ))
        .err_match(|e| {
            let msg = match e {
                ChoiceError::InvalidChoice(s) => {
                    format!("\nInvalid choice (`{}`) please try again.\n", s)
                }
            };
            Some(msg)
        })
        .get()
}

fn main() {
    let mut done: bool = false;
    while !done {
        let choice = main_menu();
        match choice {
            MainMenuChoice::ServerTime => {
                let server_time = leviathan::get_server_time()
                    .expect("Unable to retrieve server time please try again later!");
                println!("Server Time: {}\n", server_time);
            }
            MainMenuChoice::XbtInfo => {
                let future = leviathan::get_xbt_future()
                    .expect("Unable to retrieve XBT/USD information try again later!");
                println!(
                    "XBT/USD\nLast price: {last}\nIndex price: {index}\n24h volume: {vol24h}\n",
                    last = future.last,
                    index = future.index_price,
                    vol24h = future.vol24h,
                );
            }
            MainMenuChoice::OpenOrders => todo!(),
            MainMenuChoice::Quit => {
                println!("Bye!!");
                done = true;
            }
        }
    }
}
