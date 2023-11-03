#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*Callback)(void);

void p_init(Callback setup, Callback draw);

void create_window(const char *name, uint32_t width, uint32_t height);
