pub fn main() {
    let input = foo::input();
    let result = foo::Byte32(&input).unpack();
    foo::output(&result);
    let result = foo::Byte32(&input).unpack_noninline();
    foo::output(&result);
}
