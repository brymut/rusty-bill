use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please try again");
    }
    let input = buffer.trim().to_owned();
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = get_input()?;
        match input.parse::<f64>() {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    println!("Bill Name:");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };
    let bill = Bill { name, amount };
    bills.add(bill);
    println!("Bill added!");
}

fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

fn main_menu() {
    println!("");
    println!("== Bill Manager ==");
    println!("1. Add Bill");
    println!("2. View Bills");
    println!("Enter selection:");
}

fn main() {
    let mut bills = Bills::new();

    loop {
        main_menu();
        let input = match get_input() {
            Some(input) => input,
            None => continue,
        };

        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            _ => break,
        }
    }
}
