use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess. (0<=x<=10)");
    check_number();
}

fn check_number() -> () {
    let random_number: i32 = rand::thread_rng().gen_range(0..11);
    loop {
        let guess = get_line();
        let guess_number: i32 = match guess.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => -1,
        };
        println!("You guessed: {}", guess);
        if guess_number == -1 {
            println!("Please Enter the >=0 number");
            continue;
        }
        if random_number > guess_number {
            println!("Bigger");
        } else if random_number == guess_number {
            println!("Match");
            break;
        } else {
            println!("Smaller");
        }
    }
}

fn get_line() -> String {
    let empty_str = "";
    //String::new(); empty string
    let mut guess = empty_str.to_owned();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}
