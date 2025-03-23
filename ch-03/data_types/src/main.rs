fn main() {
    // Rust have 2 types of data types 1.scaler 2.compound

    // Scalar types means single values, rust have 4 scalar types
    // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

    // -------------  i means singed integres  132 and u means unsigned integers like u32
    // i32,u32,i8,u8 the numbers means bits
    // unsigned means there is not sign with number like 34
    // signed means sign with number like -323 and +323
    // i8 -> means 8 bits -(2^n-1) - 2^n-1 -> 2^8 = -128-127

    // we also can write as
    // let a = 23u8;
    // let b = 323_i32;

    // for better visualization we can use the underscore in numbers like as follows (visual_seprator)
    // let num = 1_00_000; // -> 100000

    // by default the integer in the rust is i32

    // when we compile this code in the debug mode cargo run it will show the program is panicked but
    // if we compile and run it prod. cargo run --release it will be start a new cycyle after the limit is reached 255
    // 256 ->0
    //256 -> 1 and so on this code will return 44 because there are the 44 numbers in 2nd cycle

    // this is the concept of integer overflow

    // fn random() -> u8 {
    //     200 //return 200
    // }

    // let a: u8 = random() + 100;
    // println!("The value of a is {a}");
    // ----------------------Float------------------------------------------------------------------

    // for the floating points we have the two varient and by default the floating is f64

    let float = 4.2;

    //   the second one is f32 and all 2 types are the signed numbers
    // the f64 have more double precision means it gives more decimal values then f32 which have single precision for example f64: 3.443434343432342342 and f32: 3.4342132 etc.

    //------------------------Numeric Operaters---------------------------------------------------------

    let add = 4 + 4;
    let sub = 43 - 3;
    let reminder = 5 % 2;
    let float = 5 / 3;

    // --------------------- Booleans -------------
    let tr = true;
    let fal = false;
    // -------------------- characters ---------

    let c = 'r';
    let character: char = 'f'; // explicit type annotation
    // we also can store emojis in it, every character have the 4 bytes in it
    let a = "e"; // this is the string as it is in the double quotes 

    // ===============================  Compound Types =========================
    // In rust there are two types of compund primitive types: tuples and arrays

    // ----------------- tuples -------------------

    //     The Tuple Type
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    let tuple = ("name", 25, true, 3.2);
    // destructure
    let (r, x, y, z) = tuple;

    // we can also use it as 

    println!("the 0th value is {}",tuple.0);

    println!(
        "the value of r is {r} The value of x is {x}, the value of y is {y}, the value of z is {z}"
    );

    // empty tupkle is called a unit 
    let tup = ();


    // --------------- arrays ---------------
    // arrays in rust have the fixed length

    let array = [1,2,3,4];
    let b = [9;4]; // [9,9,9,9]

    // accessing the array elements 

    let first_ele = array[0];
    println!(" The first element of the array is {first_ele}")


    println!("i8 min: {}", i8::MIN);
    println!("i8 max: {}", i8::MAX);
    println!("u8 min: {}", u8::MIN);
    println!("u8 max: {}", u8::MAX);
    println!("i32 min: {}", i32::MIN);
    println!("i32 max: {}", i32::MAX);
    println!("u32 min: {}", u32::MIN);
    println!("u32 max: {}", u32::MAX);


}

/*

To find the range of a data type in Rust, you can use the std::i8::MIN, std::i8::MAX, std::i16::MIN, std::i16::MAX, etc., or the corresponding u8::MIN, u8::MAX, u16::MIN, u16::MAX, etc., constants for signed and unsigned integers, respectively.
Here's a breakdown:
Integer Types and Their Ranges:
Signed Integers:
i8: -128 to 127
i16: -32,768 to 32,767
i32: -2,147,483,648 to 2,147,483,647
i64: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
i128: -170,141,183,797,372,951,532,832,772,800 to 170,141,183,797,372,951,532,832,772,800
isize: Depends on the architecture (e.g., 32-bit or 64-bit)

Unsigned Integers:
u8: 0 to 255
u16: 0 to 65,535
u32: 0 to 4,294,967,295
u64: 0 to 18,446,744,073,709,551,615
u128: 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455
usize: Depends on the architecture (e.g., 32-bit or 64-bit)
Floating-Point Types:
f32: Single-precision floating-point (IEEE 754)
f64: Double-precision floating-point (IEEE 754)

Other Data Types:
char: Unicode scalar value (U+0000 to U+10FFFF)
bool: true or false
(): The unit type, with a single value ()

Example:
Code

fn main() {
    println!("i8 min: {}", i8::MIN);
    println!("i8 max: {}", i8::MAX);
    println!("u8 min: {}", u8::MIN);
    println!("u8 max: {}", u8::MAX);
    println!("i32 min: {}", i32::MIN);
    println!("i32 max: {}", i32::MAX);
    println!("u32 min: {}", u32::MIN);
    println!("u32 max: {}", u32::MAX);
}

*/
