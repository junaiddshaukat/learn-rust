// Ownership with functions

fn main() {

    let num = 5;
    let result = add(num);


    let name = String::from("Junaid");
    let s:String = gives_ownership();

    let s:String = takes_and_gives_back(s);
    
    takes_ownsership(name);


    println!("The value of num is {num} and the value of result is {result}");
    println!("S = {s}");

    // println!("The value of the name is {name}") // this line gives error because we are trying to borrow a moved value 
}

fn takes_and_gives_back(s:String) -> String {
     println!("S in takes_and_gives_back {s}");
     s
}

fn gives_ownership() -> String {
    let s:String = String::from("This is a string from gives ownership");
    s
}

fn takes_ownsership(s:String) {
    println!("Inside Ownership {s}");
}

fn add(n:i32) ->i32 {
    n+4
}