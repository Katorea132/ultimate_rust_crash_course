pub fn inspect(pluralineitor: &String) {
    if pluralineitor.ends_with("s") {
        println!("Plural AF.");
    } else {
        println!("Singular AF.");
    }
}

pub fn change(changeitor: &mut String) {
    if !changeitor.ends_with("s") {
        changeitor.push_str("s");
    }
}

pub fn eat(nomnom: String) -> bool {
    nomnom.starts_with("b") && nomnom.contains("a")
}

pub fn add(num1: &i32, num2: &i32) -> i32 {
    (*num1) + (*num2)
}
