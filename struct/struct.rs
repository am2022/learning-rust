struct User{
    username: String,
    email: String,
    friend_count: i32,
}

fn main(){
    let user1 = User{
        username: String::from("test"),
        email: String::from("test@test.com"),
        friend_count: 5,
    };

    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("number of friend: {}", user1.friend_count);
}