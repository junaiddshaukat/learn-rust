// const P: i32 = 5;

fn main() {
    // for the variables like let the compiler automatically detect the type
    // let mut a = 5;

    // println!("the value of a is {a}");
    // a = 4;
    // println!("the value of a is {a}");

    // we cannot use the mut with  constant -> const and we must have to write the type with const
    // we also can define the constant on the global scope upper the main fn
    // const P:i32 = 5;
    // println!("{}", P);
    // -------  This is valid ----------
    // const THREE_HOURS_IN_SECONDS:i32 = 60*60*3;
    // println!("{THREE_HOURS_IN_SECONDS}")
    // --------- this is not valid because we can not determine the const value at runtime ------------
    // const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3 + a;

    //    -----------------------------  Shadowing  ----------------------------------------------------

    // shadowing is the use of same variable again in the program, it is different from the mutable and we use the let keyword for using it

    let abc = 32;
    println!("abc 1 : {abc}");
    // in rest of the program below abc value is used and we also can change the type of following variable
    // let abc = 32323;
    // println!("abc 2 : {abc}");

    let abc = true;
    println!("abc 2 : {abc}");
}
