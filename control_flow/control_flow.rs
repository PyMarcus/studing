
use std::io;


fn main(){

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let fibo: i32 = fibonacci(input);
    std::println!("fibo {fibo}")
}



fn fibonacci(num: i32) -> i32{
    if num > 1{
        return fibonacci(num - 1) + fibonacci(num - 2);
    }else{
        return 1;
    }
}

