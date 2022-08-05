fn floatifty() {
    let  x = 1.1;
    let y = 2.2;
    println!("x time y is {}", x * y);

}


fn printify() {
    // let is like const in JS
    // is immutable
    let my_string = "🦀";
    // my_string = "🐧" => ❌
    let mut my_mut_var = "mutable";
    println!("That my string {}", my_string);
    println!("That my mutable var {}", my_mut_var);
}




fn main() {
    floatify()
    printify()
}
