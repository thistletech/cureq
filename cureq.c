#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "./cureq.h"

int main() {
    char buffer[5000];
    char *url = "https://httpbin.org/anything";
    char *method = "POST";
    char *payload = "some payload";
    int payload_len = strlen(payload);

    char *header0[] = {"name0", "value0"};
    char *header1[] = {"name1", "value1"};
    char **headers[] = {header0, header1};

    fprintf(stderr, "Making a %s request to %s\n", method, url);
    fprintf(stderr, "Set headers: %s: %s, %s: %s\n", header0[0], header0[1], header1[0], header1[1]);
    fprintf(stderr, "====================\n");

    int read_bytes = 0;
    int status = cureq_call(method, url, headers, 2, buffer, sizeof(buffer), &read_bytes, payload, payload_len);
    if (status == -1) {
        fprintf(stderr, "Error fetching content!\n");
        return 1;
    }

    fprintf(stderr, "=> Status code: %d\n", status);
    fprintf(stderr, "=> Response: %s\n", buffer);
    return 0;
}