fn main() {
   
   // control flow -> if else
   // if else is expression so that it will return valuesp

   let call = is_even(24);
   //if call true then a=10 else 20

   let a = if call {10} else {20};
   println!("{a}");

   let x = 1;
   
   if x>2 {
    print!("true");
   }
   else {
    print!("false");
   }



// --------------- loops ------------

// loop {
//     println!("This is a forever loop, -> Infinite Loop");
// }

let mut n = 0;

 let result = loop {
    println!(" The value of n is {n}");

    if n==10 {
       break 80; 
    }

    n+=1;
};

println!("This is the last number and the result is {result}");


println!("\n This is the While loop \n");

let mut my_num = 3;

while my_num != 0 {
    println!("number is {my_num}");
    my_num-=1;
}

// for loop

println!("\n This is a for loop \n");

let arr = [1,2,3,4,5,6,7];

for elements in arr {

    println!("values {elements}");

}

// range

for x in 1..10 {
    println!("x is {x}\n");
}

for x in (1..=10).rev() {
    println!("\nx is {x}");
}




}

fn is_even(num:i32) -> bool {
    if num%2 == 0 {
       return true
    } else {
        false
    }
}
