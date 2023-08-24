use core::arch::asm;

#[allow(dead_code)]
pub const STDIN: i32 = 0;
pub const STDOUT: i32 = 1;
#[allow(dead_code)]
pub const STDERR: i32 = 2;

pub fn exit(status: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("edi") status
        );
    }

    unreachable!()
}

pub fn write(fd: i32, buf: &[u8]) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") 1,
            in("rdi") fd,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            lateout("rax") ret
        );
    }
    ret
}

pub fn read(fd: i32, buf: &mut [u8]) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") 0,
            in("rdi") fd,
            in("rsi") buf.as_mut_ptr(),
            in("rdx") buf.len(),
            lateout("rax") ret
        );
    }
    ret
}

pub fn close(fd: i32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") 3,
            in("rdi") fd,
            lateout("rax") ret
        );
    }
    ret
}

/* network related syscalls */

pub const AF_INET: i32 = 2;
pub const SOCK_STREAM: i32 = 1;

#[repr(C)]
pub struct InAddr {
    s_addr: u32, /* big endian */
}

#[repr(C)]
pub struct SockAddrIn {
    sin_family: u16,
    sin_port: u16, /* big endian */
    sin_addr: InAddr,
    sin_zero: [u8; 8],
}

pub fn socket(domain: i32, sock_type: i32, protocol: i32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") 41,
            in("rdi") domain,
            in("rsi") sock_type,
            in("rdx") protocol,
            lateout("rax") ret
        );
    }
    ret
}

pub fn bind(sockfd: i32, ip: [u8; 4], port: u16) -> i32 {
    let addr = SockAddrIn {
        sin_family: AF_INET as u16,
        sin_port: port.to_be(),
        sin_addr: InAddr {
            s_addr: u32::from_le_bytes(ip),
        },
        sin_zero: [0; 8],
    };
    let addrlen: u32 = core::mem::size_of::<SockAddrIn>() as u32; // using u32 for socklen_t

    let ret: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") 49,
            in("rdi") sockfd,
            in("rsi") &addr,
            in("rdx") addrlen,
            lateout("rax") ret
        );
    }
    ret
}

pub fn listen(sockfd: i32, backlog: i32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") 50,
            in("rdi") sockfd,
            in("rsi") backlog,
            lateout("rax") ret
        );
    }
    ret
}

pub fn accept(sockfd: i32) -> (i32, SockAddrIn) {
    let mut addr = unsafe { core::mem::zeroed::<SockAddrIn>() };
    let mut addrlen: u32 = core::mem::size_of::<SockAddrIn>() as u32; // using u32 for socklen_t

    let ret: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") 43,
            in("rdi") sockfd,
            in("rsi") &mut addr,
            in("rdx") &mut addrlen,
            lateout("rax") ret
        );
    }
    (ret, addr)
}
