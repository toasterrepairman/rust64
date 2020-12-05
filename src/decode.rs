use base64::decode;

fn decode_buf(buffer: String) {
    println!("{:?}", decode(buffer));
}