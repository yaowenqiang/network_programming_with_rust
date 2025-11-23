fn main() {
    let s = String::from("Test");
    heap_example(s);
}

fn heap_example(input: String) {
    let mystr = input;
    //let _other = mystr;
    let _other = mystr.clone();
    println!("{mystr}");
}
