#include <stdio.h>
#include "../bindings.h"

void setup () {
    create_window("Hello World", 800, 600);
}

void draw () {
    printf("Drawing\n");
}

int main() {
    p_init(setup, draw);
    p_run();
    return 0;
}
