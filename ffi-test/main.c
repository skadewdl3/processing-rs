#include <stdio.h>
#include "../bindings.h"

void setup () {
    create_window("Hello World", 800, 600);
}

void draw () {
}

void mousePressed (WindowEvent event) {
    printf("Mouse pressed at: (%f, %f)\n", event.mouse.x, event.mouse.y);
}

int main() {
    p_init(setup, draw);
    p_on(PMousePressed, mousePressed);
    p_run();
    return 0;
}
