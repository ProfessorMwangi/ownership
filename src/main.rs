fn main() {
    let s1 = String::from("Rust Rules!");
    let s2 = s1; // s1 is moved to 
    println!("{}", s2); // s1 is no longer valid since the ownership has moved to s2
}

