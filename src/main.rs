use std::collections::HashMap;
use std::io;

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

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        if let Some(bill) = self.inner.get_mut(name) {
            bill.amount = amount;
            true
        } else {
            false
        }
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
    println!("Amount (or Enter to cancel):");
    loop {
        let input = get_input()?;
        match input.parse::<f64>() {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    println!("Bill Name (or Enter to cancel):");
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

fn remove_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to remove (or Enter to cancel):");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    if bills.remove(&name) {
        println!("Bill removed");
    } else {
        println!("Bill not found");
    }
}

fn edit_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to edit (or Enter to cancel):");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };
    if bills.update(&name, amount) {
        println!("Bill updated");
    } else {
        println!("Bill not found");
    }
}

fn view_bills_menu(bills: &Bills) {
    let all_bills = bills.get_all();
    if all_bills.is_empty() {
        println!("No bills found.");
    } else {
        for bill in all_bills {
            println!("{:?}", bill);
        }
    }
}

fn main_menu() {
    println!("");
    println!("== Bill Manager ==");
    println!("1. Add Bill");
    println!("2. View Bills");
    println!("3. Remove Bill");
    println!("4. Edit Bill");
    println!("Enter selection (anything else to quit):");
}

fn main() {
    let mut bills = Bills::new();

    loop {
        main_menu();
        let input = match get_input() {
            Some(input) => input,
            None => break,
        };

        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => edit_bill_menu(&mut bills),
            _ => break,
        }
    }
    println!("Goodbye!");
}
