use std::io;
use sha2::{Sha512, Digest};
use md5::Md5;
use sha1::Sha1;

fn hash_sha512(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn hash_md5(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn hash_sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn hash_sha256(input: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn main() {
    println!("Digite uma senha: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    let input = input.trim();

    let sha512_hash = hash_sha512(input);
    let md5_hash = hash_md5(input);
    let sha1_hash = hash_sha1(input);
    let sha256_hash = hash_sha256(input);

    println!("Input: {}", input);
    println!("SHA-512 Hash: {}", sha512_hash);
    println!("MD5 Hash: {}", md5_hash);
    println!("SHA-1 Hash: {}", sha1_hash);
    println!("SHA-256 Hash: {}", sha256_hash);
}