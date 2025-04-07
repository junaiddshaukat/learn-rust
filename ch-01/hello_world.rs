use std::io;

fn main(){ // main function
    println!("Hello World!"); // !macro -> used to print

    println!("Enter a number:");

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read line");

let num: i32 = input.trim().parse().expect("Please type a number!");
let result = num * (num + 1);

println!("The result of {} multiplied by {} is {}", num, num + 1, result);

let a = 5;
let res = a*4-3;

println!("the valuse of a is {a} and the value of res is {res}");

}
