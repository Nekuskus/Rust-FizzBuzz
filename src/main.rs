fn main() {
    println!("{}", (1..101).into_iter().map(|num| {
        match num {
            _ if num % 15 == 0 => String::from("FizzBuzz"),
            _ if num % 5 == 0 => String::from("Buzz"),
            _ if num % 3 == 0 => String::from("Fizz"),
            _ => num.to_string()
        }
    }).collect::<Vec<String>>().join(" "));
}
