#![allow(non_snake_case)]
use std::io;

fn main() {
    println!("Hi you are in the identityCard project ! \nYou are here for create your identity card !");

    println!("Please enter your name : ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Please enter your surname : ");
    let mut surname = String::new();
    io::stdin().read_line(&mut surname).expect("Failed to read line");

    println!("Please enter your age : ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");

    println!("Please enter your address : ");
    let mut address = String::new();
    io::stdin().read_line(&mut address).expect("Failed to read line");

    println!("Please enter your phone number : ");
    let mut phone_number = String::new();
    io::stdin().read_line(&mut phone_number).expect("Failed to read line");

    println!("Please enter your email : ");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to read line");

    println!("Please enter your job : ");
    let mut job = String::new();
    io::stdin().read_line(&mut job).expect("Failed to read line");

    println!("Please enter your hobbies : ");
    let mut hobbies = String::new();
    io::stdin().read_line(&mut hobbies).expect("Failed to read line");

    let identityCard = (
        name,
        surname,
        age,
        address,
        phone_number,
        email,
        job,
        hobbies,
    );
    let (name, surname, age, address, phone_number, email, job, hobbies) = identityCard;

    println!("Here is your identity card : \n \t Name : {} \t Surname : {} \t Age : {} \t Address : {} \t Phone number : {} \t Email : {} \t Job : {} \t Hobbies : {}", name, surname, age, address, phone_number, email, job, hobbies);
}
