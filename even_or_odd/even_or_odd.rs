use std::io;

fn main(){
    let mut s_num = String::new();
    io::stdin().read_line(&mut s_num).expect("invalid input!");
    
    let num: i64 = s_num.trim().parse().expect("invalid input!");

    if num % 2 == 0{
        println!("even!");
    }
    else{
        println!("odd!");
    }
}