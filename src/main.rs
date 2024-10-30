use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
}

impl Contact {
    fn new(name: String, phone: String) -> Contact {
        Contact { name, phone }
    }
}

fn main() {
    let mut contacts: HashMap<String, Contact> = HashMap::new();

    loop {
        println!("chose your option");
        println!("1- Add Contact");
        println!("2- list Contacts");
        println!("3- exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to read input");

        match choice.trim() {
            "1" => add_contact(&mut contacts),
            "2" => list_of_contacts(&contacts),
            _ => {
                println!("Exiting");
                break;
            }
        }
    }
}

fn add_contact(contacts: &mut HashMap<String, Contact>) {
    println!("Enter name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("there is problem with name");
    let name = name.trim().to_string();

    println!("Enter Phone:");
    let mut phone = String::new();
    io::stdin()
        .read_line(&mut phone)
        .expect("there is pronlem with phone");
    let phone = phone.trim().to_string();

    let contact = Contact::new(name.clone(), phone);
    contacts.insert(name, contact);

    println!("data in succcessfully");
}

fn list_of_contacts(contacts: &HashMap<String, Contact>) {
    if contacts.is_empty() {
        println!("there is no contact");
    } else {
        println!("Contacts:");
        for contact in contacts.values() {
            println!("{:?}", contact)
        }
    }
}
