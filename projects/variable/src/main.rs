#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    nb_connections: u32,
}

fn main() {
    // let x = 5;
    let mut x: i32 = 5;
    println!("x = {x}");

    x = 6;
    println!("x = {x}");

    let username: String = String::from("u.ser");
    let email: String = String::from("a@a.a");

    let mut user1: User = build_user(username, email);

    print_user(&user1);

    user1.username = String::from("u.ser_");

    print_user(&user1);

    let user2 = User {
        email: String::from("b@b.b"),
        ..user1
    };

    print_user(&user2);

    // Won't compile because user1 has borowed values
    // user1.active = false;
    // print_user(&user1);
}

fn print_user(user: &User) {
    println!(
        "{} \n\t email: {} \n\t nb_connection(s): {} \n\t active: {}",
        user.username, user.email, user.nb_connections, user.active
    );
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        nb_connections: 1,
    }
}
