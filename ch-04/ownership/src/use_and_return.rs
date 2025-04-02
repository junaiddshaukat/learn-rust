fn main(){
    let s:String = String::from("Junaid");
    let (s, length) = calculate_length(s); //shadowing s
    println!("The length of string {s} is {length}");
}

fn calculate_length(s: String) -> (String,usize) {
    let l = s.len();
    (s,l)
}

// this is the tedious work and we can use the refrences and borrowing for it 