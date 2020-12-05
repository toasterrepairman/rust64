use base64::encode;

pub fn encode_buf(buffer: String) {
    println!("{}", encode(buffer));
}