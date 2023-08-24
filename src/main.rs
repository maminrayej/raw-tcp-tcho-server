#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();

    sys::exit(0)
}

#[macro_use]
mod mac;
mod aux;
mod fmt;
mod net;
mod sys;

fn main() {
    let listener = net::TcpListener::new([127, 0, 0, 1], 8080);

    let stream = listener.accept();

    let mut buf = [0; 1024];
    loop {
        let nbytes = stream.read(&mut buf);

        if nbytes == 0 {
            println!("Reached EOF");
            break;
        } else if nbytes < 0 {
            println!("Read failed: {}", -nbytes);
            break;
        }

        let nbytes = stream.write(&buf[..nbytes as usize]);
        if nbytes < 0 {
            println!("Write failed: {}", -nbytes);
            break;
        }
    }
}
