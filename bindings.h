#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*Callback)(void);

typedef struct Vertex {
  int32_t position[2];
} Vertex;

void p_init(Callback setup, Callback draw);

void create_window(const char *name, uint32_t width, uint32_t height);

void triangle(struct Vertex v1, struct Vertex v2, struct Vertex v3);
