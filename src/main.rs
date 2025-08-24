fn main() {
    let s1 = String::from("hello");
    take_ownership(s1);
}

fn take_ownership(string: String) {
    println!("{} this is from take ownership fn.", string);
}