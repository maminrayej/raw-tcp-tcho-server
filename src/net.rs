use crate::sys;

pub struct TcpListener {
    fd: i32,
}
impl TcpListener {
    pub fn new(ip: [u8; 4], port: u16) -> Self {
        let socket = sys::socket(sys::AF_INET, sys::SOCK_STREAM, 0);
        if socket < 0 {
            println!("Failed to create socket: {}", -socket);
            sys::exit(1);
        }

        let result = sys::bind(socket, ip, port);

        if result < 0 {
            println!("Failed to bind the socket to: {:?}:{}", ip, port);
            sys::exit(1);
        }

        let result = sys::listen(socket, 10);

        if result < 0 {
            println!("Failed to listen: {}", -result);
            sys::exit(1);
        }

        TcpListener { fd: socket }
    }

    pub fn accept(&self) -> TcpStream {
        let (fd, _) = sys::accept(self.fd);

        if fd < 0 {
            println!("Failed to accept: {}", -fd);
            sys::exit(1);
        }

        TcpStream { fd }
    }
}
impl Drop for TcpListener {
    fn drop(&mut self) {
        sys::close(self.fd);
    }
}

pub struct TcpStream {
    fd: i32,
}
impl TcpStream {
    pub fn read(&self, buf: &mut [u8]) -> isize {
        sys::read(self.fd, buf)
    }
    pub fn write(&self, buf: &[u8]) -> isize {
        sys::write(self.fd, buf)
    }
}
impl Drop for TcpStream {
    fn drop(&mut self) {
        sys::close(self.fd);
    }
}
