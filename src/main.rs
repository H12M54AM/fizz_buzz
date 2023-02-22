fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for num in numbers {
        if num % 15 == 0 {
            println!("Fizz Buzz");
        } else if num % 3 == 0 {
            println!("{} Fizz", num);
        } else if num % 5 == 0 {
            println!("{} Buzz", num);
        } else {
            println!("{}", num);
        }
    }
}

