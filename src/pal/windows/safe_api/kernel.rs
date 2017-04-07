use kernel32::{FormatMessageW, GetLastError, LocalFree};
use winapi::minwindef::*;
use winapi::winbase::*;
use std::ptr;
use std::mem;
use std::slice::from_raw_parts;

pub fn get_last_error() -> DWORD {
    unsafe { GetLastError() }
}

pub fn error_message(error_num: DWORD) -> String {
    let mut out_buf: *const u16 = ptr::null_mut();
    let bufLen = unsafe {
        FormatMessageW(FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM,
                       ptr::null(),
                       error_num,
                       0,
                       mem::transmute(&out_buf),
                       0,
                       ptr::null_mut())
    };
    let slice = unsafe { from_raw_parts(out_buf, bufLen as usize) };
    let result = String::from_utf16_lossy(slice);
    unsafe { LocalFree(mem::transmute(out_buf)) };
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn error_message_test() {
        assert_ne!(error_message(0).len(), 0);
    }
}