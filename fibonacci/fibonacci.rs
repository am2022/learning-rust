use std::io;

fn main(){
    let mut s_num = String::new();
    println!("enter the number:");
    io::stdin().read_line(&mut s_num).expect("invalid input!");

    let num: i64 = s_num.trim().parse().expect("invalid input!");

    let mut f1: u128 = 1;
    let mut f2: u128 = 1;
    let mut fsum: u128;

    println!("1: {f1}");
    println!("2: {f2}");

    for i in 1..(num-1){
        fsum = f1 + f2;
        f1 = f2;
        f2 = fsum;
        println!("{}: {fsum}", i+2);
    }
}