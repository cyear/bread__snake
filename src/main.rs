use bread_snake::*;
fn main() {
    let args = bargs();
    let buf = brean(&args);
    println!("Buf len: {}", buf.len());
    if buf.len() < 5000 {
        println!("Bean: {:#?}", String::from_utf8(buf).unwrap());
    }
}
