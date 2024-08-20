use ureq;
use std::ffi::{c_int, c_uchar, c_char, CStr};
use std::io::Read;

#[no_mangle]
/// Make a GET request to the given URL
///
/// @fn ureq_get
///
/// @param method: The HTTP method to use (e.g. "GET", "POST", "PUT", "DELETE")
/// @param url: The URL to make the GET request to
/// @param headers: A pointer to an array of headers. E.g. [["key, "value"], ["key2", "value2]]
/// @param headers_count: The number of headers in the headers array
/// @param ret_buffer: A pointer to a buffer to store the response
/// @param max_ret_buffer: The size of the buffer in bytes
/// @param ret_buffer_read: The number of bytes read into the buffer
/// @param payload: A pointer to a buffer containing the payload to send (can be null)
/// @param payload_len: The length of the payload buffer
///
/// @return The HTTP status code returned, or -1 if an error occurred
///
pub extern "C" fn cureq_call(method: *const c_char, url: *const c_char, headers: *const *const *const c_char, headers_count: c_int, ret_buffer: *mut c_uchar, max_ret_buffer: c_int, ret_buffer_read: *mut c_int, payload: *mut c_uchar, payload_len: c_int) -> c_int {
    if method.is_null() || url.is_null() || ret_buffer.is_null() || max_ret_buffer == 0 || (payload.is_null() && payload_len > 0) {
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
    let method = unsafe {
        match CStr::from_ptr(method).to_str() {
            Ok(s) => s,
            Err(_) => {
                return -1;
            }
        }
    };

    let mut req = ureq::request(method, url);

    // parse headers and add to request
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

    // commit request, with or without payload
    let resp = if payload.is_null() {
        req.call()
    } else {
        let payload = unsafe { std::slice::from_raw_parts(payload, payload_len as usize) };
        let payload_reader = std::io::Cursor::new(payload);
        req.send(payload_reader)
    };

    let resp = match resp {
        Ok(r) => r,
        Err(_) => {
            return -1;
        }
    };

    // read response into buffer
    let ret_buffer = unsafe { std::slice::from_raw_parts_mut(ret_buffer, max_ret_buffer as usize) };
    let status = resp.status();
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
        if total_bytes_read >= max_ret_buffer as usize {
            break;
        }
    };

    unsafe {
        *ret_buffer_read = total_bytes_read as c_int;
    }
    return status as c_int;
}
