fn main() {
    let number_1 = 6;

    if number_1 % 4 == 0 {
        println!("number_1 is divisible by 4");
    } else if number_1 % 3 == 0 {
        println!("number_1 is divisible by 3");
    } else if number_1 % 2 == 0 {
        println!("number_1 is divisible by 2");
    } else {
        println!("number_1 is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number_2 = if condition { 5 } else { 6 };

    println!("number_2 is: {number_2}");
}
