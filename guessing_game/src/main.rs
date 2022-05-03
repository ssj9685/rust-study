use rand::Rng;
macro_rules! get_line {
    () => {
        get_line_func("")
    };
    ($title: expr) => {
        get_line_func($title)
    };
}
fn main() {
    let count: i32 = str_to_int(get_line!("Input Guess Number Range"));
    println!("Please input your guess. (0<=x<={})", count);
    check_number(count);
}

fn check_number(count: i32) -> () {
    let random_number: i32 = rand::thread_rng().gen_range(0..count);
    loop {
        let guess = str_to_int(get_line!());
        println!("You guessed: {}", guess);
        if guess == -1 {
            println!("Please Enter the >=0 number");
            continue;
        }
        if random_number > guess {
            println!("Bigger");
        } else if random_number == guess {
            println!("Match");
            break;
        } else {
            println!("Smaller");
        }
    }
}

fn get_line_func(title: &str) -> String {
    println!("{}", title);
    let empty_str = "";
    //String::new(); empty string
    let mut guess = empty_str.to_owned();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}

fn str_to_int(str: String) -> i32 {
    return match str.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => -1,
    };
}
