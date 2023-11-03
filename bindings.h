#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum MouseButton {
  Left,
  Right,
  Middle,
  None,
} MouseButton;

typedef enum WindowEvents {
  POnClick,
  POnMousePressed,
  POnMouseReleased,
  POnMouseMoved,
} WindowEvents;

typedef void (*Callback)(void);

typedef struct MouseEvent {
  float x;
  float y;
  enum MouseButton button;
  enum WindowEvents event_type;
} MouseEvent;

typedef struct KeyboardEvent {
  uint32_t key;
} KeyboardEvent;

typedef union WindowEvent {
  struct MouseEvent mouse_event;
  struct KeyboardEvent keyboard_event;
} WindowEvent;

typedef void (*EventHandler)(union WindowEvent);

typedef struct Vertex {
  int32_t position[2];
} Vertex;

void p_init(Callback setup, Callback draw);

void p_run(void);

void p_on(enum WindowEvents event, EventHandler handler);

void create_window(const char *name, uint32_t width, uint32_t height);

void triangle(struct Vertex v1, struct Vertex v2, struct Vertex v3);
