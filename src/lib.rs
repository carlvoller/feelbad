
use libc::{c_int, c_void, size_t, ssize_t, STDERR_FILENO};
use redhook::{hook, real};

fn should_colorise(fd: c_int) -> bool {
    fd == STDERR_FILENO
}

pub static START: &str = concat!("\x1b[31m", "\0");
pub static END: &str = concat!("\x1b[0m", "\0");

hook! {
    unsafe fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t => my_write {
        if count == 0 {
            return 0;
        }

        if should_colorise(fd) {
            let written = real!(write)(fd, "\x1b[31m".as_ptr() as *const c_void, 5);
            if written <= 0 {
                return 0;
            }
            if written < START.len() as isize - 1 {
                real!(write)(fd, "\x1b[0m ".as_ptr() as *const c_void, 5);
                return 0;
            }
        }

        let written = real!(write)(fd, buf, count);

        if written > 0 && should_colorise(fd) {
            real!(write)(fd, "\x1b[0m".as_ptr() as *const c_void, 5);
        }

        written
    }
}
