pub fn out(s: &str) {
    println!("{s}");
}
pub fn warn(s: &str) {
    out(&s);
}
pub fn err(s: &str) {
    eprintln!("{}", &s);
}