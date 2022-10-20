use std::io;

fn main() {
    println!("welcom to the guessing game !!");

    let secret_number = rand::Rng::thread_rng().gen_range(1..=100);

    println!("please enter a number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("epic failure");

    println!("you enter :: {guess}");
}
