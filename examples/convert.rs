#![feature(string_from_utf8_lossy_owned)]

pub fn main() {
    todo!()
}

pub fn a() {
    pub fn convert(bytes: &[u8]) -> String {
        String::from_utf8_lossy_owned(std::vec::Vec::from(bytes))
    }

    let foo: &[u8] = std::hint::black_box("hello".as_bytes());
    convert(foo);
}

pub fn b() {
    pub fn convert(bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes).into_owned()
    }
    let foo: &[u8] = std::hint::black_box("hello".as_bytes());
    convert(foo);
}
