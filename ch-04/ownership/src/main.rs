fn main() {
    let mut s = String::from("Hello");

    s.push_str(" WOrld");

    println!("the value of the S = {s}")

    // String

    let s1 = String::from("This is S1 ");

    let s2 = s1; // gives error because we can't do that
    let mut s2 = s1.clone(); // this is expensive 
    s2.push_str("This is push");

    println!("S1 = {s1}");
    println!("S2 = {s2}");
}
