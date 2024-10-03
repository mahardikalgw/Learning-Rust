fn main() {

    // has immutable variable
    let x: i32 = -4;
    if x >= 0 {
        println!("x is not negative");
    } else {
        println!("x is negative");
    }


    // you can initiate variable with mut keyword to explain the variable has mutable to change value possessed
    let mut i = 1;
    while i <= 5 {
        println!("{}", i);
        i += 1;
    }
}
