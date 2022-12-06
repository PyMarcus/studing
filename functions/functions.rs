

fn main(){
    /*
    Functions are the prevalent in Rust code.
    */

    function();
    function_with_arguments(33);
    
    let x = {
        let y = 2;
        y + 1
    };

    std::println!("X: {x}"); // 3, its how a create a function,because rust is a expressive language
    let z: i32 = fucntion_with_return();
    let k: char = function_with_return2();
    std::println!("{z} {k}");
}



fn function(){
    std::println!("Print");
}


fn function_with_arguments(number: u32){
    std::println!("Number {number}");
}


fn fucntion_with_return() -> i32 {
    5
}


fn function_with_return2() -> char {
    return 'a';
}

