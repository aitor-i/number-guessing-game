use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Gues the number!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop{ 

        println!("Please enter a your gues: ");

        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Your guess is {}", guess);


        let guess: u8 = match guess.trim().parse() { 
            Ok(num) => num,
            Err(_) => { 
                println!("Please introduce a number between 0 and 100");
                continue;
            }

        };

        match guess.cmp(&secret_number){ 
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal =>{ 
                println!("You win!!");
                break;
            }
        }
    }
}
