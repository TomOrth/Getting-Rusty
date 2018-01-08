fn main() {
    let mut s = String::new();

    let data = "initial";

    let s = data.to_string(); //literal to String
    let s = String::from("literal"); //works same as above
    s.push_str(" strings");
    println!("{}", s);
}