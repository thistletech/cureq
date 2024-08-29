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
    const char *header1[] = {"name1", "value1"};
    const char *const *headers[] = {header0, header1};
    int headers_count = 2;

    fprintf(stderr, "Making a %s request to %s\n", method, url);
    fprintf(stderr, "Set headers: %s: %s, %s: %s\n", header0[0], header0[1], header1[0], header1[1]);
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