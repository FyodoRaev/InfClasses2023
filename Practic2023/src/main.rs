fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    return s;
}
fn main() {
    let s = read_line();
    let bl: bool = false;
    if s == "y" {
        bl == true;
        println!("{}", bl as String);
    }
}
