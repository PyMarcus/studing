
// const are not immutable by default, they are ever immutable
// cannot use mut with const
// they can be use in  global scope
const THREE: u32 = 3;


fn main(){
    let xv = 5; // imutable integer (0, 100) variable
    let mut ale  = 3;
    ale = 2;
    std::print!("{ale} {xv}");

    std::print!("{THREE}");
    // shadowing: is the fact of can be redeclare a variable
    let v: i32 = 3;
    std::println!("before scope {v}");
    {
        let v:i32 = 44;  // this part is equal the js
        std::println!("Inner scope {v}");
    }

    std::println!("After scope {v}");


    // data types (scalar types)
    // integer:
    let x: i32 = 3; // signed type (32 bits)
    let x: u8 = 1; // unsigned type (8bits)
    std::println!("{x}");

    let x: i32 = 3_000; // 3000
    let x: i32 = 0xff;
    let x:i32 = 0b1111;
    let x:u8 = b'A'; // bytes

    // float type
    let f64 = 2.9;
    let f32: f32 = 3.3;
    std::println!("{f32}");

    // operations
    let x: f32 = (((x + 3) - (1 * 2)) / (20 % 11)).into(); // convert integer type to float
    std::println!("{x}");

    let x: bool = true;
    let x: char = 'a';

    // compound types
    let tuple_type: (i32, f64, u8) = (1, 2.3, b'a');  // fix size, variables type

    //access:
    // destructure:
    let (a, b, c) = tuple_type;

    let z: i32 = tuple_type.0;

    std::println!("{a} {z}");

    // array
    let a: [i32; 4] = [1, 2, 3, 4];  // fix size and same type
    // they are allocated in stack rather than heap

    // access:
    let x = a[0];

    std::println!("Size of array: {} items", a.len());
    let z: usize = std::mem::size_of_val(&x); // how much in mem
    std::println!("Size in the memory {z} bytes");

}
