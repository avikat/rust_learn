use std::io;

use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number\n");
    
    let secret_number = rand:: thread_rng().gen_range(1..101);
    loop {
    println!("Please enter te number\n");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    let guess:u32 = match guess.trim().parse()
    {
        Ok(num) => num,
        Err(_) => continue
    };
    // println!("Secret Number is : {}",secret_number);
    println!("You guessed: {}",guess);

    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("Too Small"),
        Ordering::Equal => {println!("you win!");
                            break;}
        Ordering::Greater => println!("to Big"),
    }

    


    }
}
