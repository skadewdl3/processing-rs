#include <stdio.h>
#include "../bindings.h"

void setup () {
    create_window("Hello World", 800, 600);
}

void draw () {
}

void mouse_moved (WindowEvent event) {
    printf("Mouse position: (%f, %f)\n", event.mouse_event.x, event.mouse_event.y);
}

int main() {
    p_init(setup, draw);
    p_on(POnMouseMoved, mouse_moved);
    p_run();
    return 0;
}
