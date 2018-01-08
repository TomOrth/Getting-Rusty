struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn activeAlot (&self) -> bool {
        self.sign_in_count > 3
    }
}

fn main() {

    let mut user1 = build_user("someone@emal.com", "someone");

    user1.email = String::from("anotherone@gmail.com");
    println!("{}", user1.activeAlot())
}

fn build_user(email: String, username: String) -> User {
    User {
        email, //shorthand init since variable matches function name
        username,
        active: true,
        sign_in_count: 1,
    }
}