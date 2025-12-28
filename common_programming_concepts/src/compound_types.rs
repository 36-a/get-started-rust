fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y); // 6.4

    println!("The value of tup1 is: {}", tup.0); // 500

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of array is: {}", a[0]); // 1
}
