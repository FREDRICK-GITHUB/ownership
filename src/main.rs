fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = gives_and_takes_back(s2);
    println!("the value of s1 is: {} and s3 is: {}", s1, s3);
  
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn gives_and_takes_back(a_string: String) -> String {
    a_string
}
