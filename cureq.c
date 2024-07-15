#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "./cureq.h"

int main() {
    char buffer[5000];
    char *url = "https://httpbin.org/anything";

    char *header0[] = {"name0", "value0"};
    char *header1[] = {"name1", "value1"};
    char **headers[] = {header0, header1};

    int success = ureq_get(url, headers, 2, buffer, sizeof(buffer));
    if (success == -1) {
        fprintf(stderr, "Buffer size is too small!\n");
        return 1;
    }

    printf("%.*s\n", strlen(buffer), buffer);
    return 0;
}