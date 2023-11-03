#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum MouseButton {
  LeftMouseButton,
  RightMouseButton,
  MiddleMouseButton,
  NoneMouseButton,
} MouseButton;

typedef enum WindowEvents {
  PMousePressed,
  PMouseReleased,
  PMouseMoved,
  PMouseWheel,
  PMouseDraged,
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
  struct MouseEvent mouse;
  struct KeyboardEvent keyboard;
} WindowEvent;

typedef void (*EventHandler)(union WindowEvent);

typedef struct Vertex {
  int32_t position[2];
} Vertex;

void p_init(Callback setup, Callback draw);

void p_run(void);

void p_on(enum WindowEvents event, EventHandler handler);

void create_window(const char *name, uint32_t width, uint32_t height);

float mouseX(void);

float mouseY(void);

void triangle(struct Vertex v1, struct Vertex v2, struct Vertex v3);
