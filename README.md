# cureq

Small C wrapper for [ureq](https://crates.io/crates/ureq). Outputs a static lib that can be versy easily linked to a C project.

Example usage:
```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "./cureq.h"

int main() {
    unsigned char buffer[5000];
    char *url = "https://httpbin.org/anything";
    char *method = "POST";
    char *payload = "some payload";
    int payload_len = strlen(payload);

    const char *header0[] = {"name0", "value0"};
    const char *const *headers[] = {header0};
    int headers_count = 1;

    fprintf(stderr, "Making a %s request to %s\n", method, url);
    fprintf(stderr, "====================\n");

    int read_bytes = 0;
    int status = cureq_call(method, url, headers, headers_count, buffer, sizeof(buffer), &read_bytes, (unsigned char *) payload, payload_len);
    if (status == -1) {
        fprintf(stderr, "Error fetching content!\n");
        return 1;
    }

    fprintf(stderr, "=> Status code: %d\n", status);
    fprintf(stderr, "=> Response: %s\n", buffer);
    return 0;
}
```

Prototype:
```c
/**
 * Perform a request to the given URL
 *
 * @fn ureq_get
 *
 * @param method: The HTTP method to use (e.g. "GET", "POST", "PUT", "DELETE")
 * @param url: The URL to make the GET request to
 * @param headers: A pointer to an array of headers. E.g. [["key, "value"], ["key2", "value2]]
 * @param headers_count: The number of headers in the headers array
 * @param ret_buffer: A pointer to a buffer to store the response
 * @param max_ret_buffer: The size of the buffer in bytes
 * @param ret_buffer_read: The number of bytes read into the buffer
 * @param payload: A pointer to a buffer containing the payload to send (can be null)
 * @param payload_len: The length of the payload buffer
 *
 * @return The HTTP status code returned, or -1 if an error occurred
 *
 */
int cureq_call(const char *method,
               const char *url,
               const char *const *const *headers,
               int headers_count,
               unsigned char *ret_buffer,
               int max_ret_buffer,
               int *ret_buffer_read,
               unsigned char *payload,
               int payload_len);
```

Build:
```sh
# requires make, rust, cargo, cbindgen, and a C compiler
; make
```