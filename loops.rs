fn main() {
    loop {
        println!("I am a loop");
        break;
    }
    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {  
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
    for (x = 0; x < 10; x++) {
        printf( "%d\n", x );
    }
    for x in 0..10 {
        println!("{}", x); // x: i32
    }
}