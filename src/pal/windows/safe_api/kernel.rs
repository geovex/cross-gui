use winapi::shared::minwindef::*;
use winapi::um::errhandlingapi::{GetLastError};
use winapi::um::winbase::{FormatMessageW, LocalFree, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_ALLOCATE_BUFFER};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::minwindef::HINSTANCE;
use std::ptr;
use std::mem;
use std::slice::from_raw_parts;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// basically `kernel32::GetLastError` wrapper
#[allow(dead_code)]
pub fn get_last_error() -> DWORD {
    unsafe { GetLastError() }
}
/// converts [`get_last_error()`](fn.get_last_error.html) result to error message in current locale
#[allow(dead_code)]
pub fn error_message(error_num: DWORD) -> String {
    let out_buf: *const u16 = ptr::null_mut();
    let buf_len = unsafe {
        FormatMessageW(FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM,
                       ptr::null(),
                       error_num,
                       0,
                       mem::transmute(&out_buf),
                       0,
                       ptr::null_mut())
    };
    let slice = unsafe { from_raw_parts(out_buf, buf_len as usize) };
    let result = String::from_utf16_lossy(slice);
    unsafe { LocalFree(mem::transmute(out_buf)) };
    result
}

pub fn get_module_handle(name: Option<&str>) -> HINSTANCE
{
    if let Some(name) = name {
        let name: Vec<u16> = OsStr::new(name).encode_wide().collect();
        unsafe { GetModuleHandleW(name.as_ptr()) }
    } else { 
        unsafe { GetModuleHandleW(ptr::null_mut()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn error_message_test() {
        assert_ne!(error_message(0).len(), 0);
    }
}