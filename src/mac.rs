macro_rules! println {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        let mut buffer = crate::fmt::Buffer::<1024>::new();
        let _ = write!(&mut buffer, $($arg)*);
        crate::fmt::print_bytes(buffer.as_bytes());
        crate::fmt::print_bytes(b"\n");
    };
}
