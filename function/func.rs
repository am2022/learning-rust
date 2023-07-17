fn hello(){
    println!("hello world!");
}

fn max(x: i64, y: i64) -> i64{
    if x < y{
        return y;
    }
    else{
        return x;
    }
}

fn main(){
    let a: i64 = 16;
    let b: i64 = 18;

    hello(); //prints hello world
    let max_ab = max(a, b); //returns bigger number
    println!("max:{max_ab}");
}