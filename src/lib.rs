use ureq;
use std::ffi::{c_int, c_uchar, c_char, CStr};
use std::io::Read;

fn parse_resp(resp: Result<ureq::Response, ureq::Error>, ret_buffer: *mut c_uchar, ret_buffer_size: c_int) -> c_int {
    let resp = match resp {
        Ok(r) => r,
        Err(_) => {
            return -1;
        }
    };
    let ret_buffer = unsafe { std::slice::from_raw_parts_mut(ret_buffer, ret_buffer_size as usize) };
    let mut reader = resp.into_reader();
    let mut total_bytes_read = 0;
    loop {
        let bytes_read = match reader.read(&mut ret_buffer[total_bytes_read..]) {
            Ok(n) => n,
            Err(_) => {
                return -1;
            }
        };
        if bytes_read == 0 {
            break;
        }
        total_bytes_read += bytes_read;
        if total_bytes_read >= ret_buffer_size as usize {
            break;
        }
    };
    total_bytes_read as c_int
}


#[no_mangle]
/// Make a GET request to the given URL
///
/// @fn ureq_get
///
/// @param url: The URL to make the GET request to
/// @param headers: A pointer to an array of headers. E.g. [["key, "value"], ["key2", "value2]]
/// @param headers_count: The number of headers in the headers array
/// @param ret_buffer: A pointer to a buffer to store the response
/// @param ret_buffer_size: The size of the buffer in bytes
///
/// @return The number of bytes read into the buffer, or -1 if an error occurred
///
pub extern "C" fn ureq_get(url: *const c_char, headers: *mut *mut *mut c_char, headers_count: c_int, ret_buffer: *mut c_uchar, ret_buffer_size: c_int) -> c_int {
    if url.is_null() || ret_buffer.is_null() || ret_buffer_size == 0 {
        return -1;
    }
    let url = unsafe {
        match CStr::from_ptr(url).to_str() {
            Ok(s) => s,
            Err(_) => {
                return -1;
            }
        }
    };

    let mut req = ureq::get(url);

    if headers_count > 0 && !headers.is_null() {
        let headers = unsafe { std::slice::from_raw_parts(headers, headers_count as usize) };
        for i in 0..(headers_count as usize) {
            let header = unsafe { std::slice::from_raw_parts(headers[i], 2 as usize) };
            let name = unsafe {
                match CStr::from_ptr(header[0]).to_str() {
                    Ok(s) => s,
                    Err(_) => {
                        return -1;
                    }
                }
            };
            let value = unsafe {
                match CStr::from_ptr(header[1]).to_str() {
                    Ok(s) => s,
                    Err(_) => {
                        return -1;
                    }
                }
            };
            req = req.set(name, value);
        }
    }

    parse_resp(req.call(), ret_buffer, ret_buffer_size)
}
