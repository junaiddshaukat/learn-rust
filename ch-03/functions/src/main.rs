fn main() {
    println!("Hello, world!");

    my_function(5); // function call | 5 is argument 

    let y = {
        let x = 4;
        x + 2 // it has no ; so that it is a expression that return a value
    };
    print!("The value of y is {y}\n");

    let add = add(3, 4);
    println!("the value of add is {add}");

}

// rust use the snake case -> another_function()
fn my_function(x: i32) {
    //parameters
    println!("This is my function {x}"); // function decleration
}

// expressions and statements in rust

//functions with return values

fn add(x: i32,y:i32) -> i32 {
     x+y
} 