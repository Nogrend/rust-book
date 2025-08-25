fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let message = "The temperature today is:";

    let t = ([1, 2], [3; 4]);

    let (a, b) = t;

    println!("{}", a[0] + t.1[0]);
}
