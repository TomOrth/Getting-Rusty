fn main() {
    print_number(5);
    println!("{}", add_one(5));
    let f: fn(i32) -> i32 = add_one;
    println!("{}",f(5)); //function pointer example
}

fn print_number(x: i32) { //functions have to have types in the params
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x+1
}
