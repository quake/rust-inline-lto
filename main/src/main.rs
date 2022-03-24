pub fn main() {
    let b = foo::Byte32(&[0; 32]);
    let a = b.unpack();
    std::process::exit(a.len() as i32);
}