# Static build for Linux/X11

# Usage:
#   just            # builds static binary `noise`
#   just static     # same as default
#   just run        # runs ./noise
#   just clean      # removes binary

cc := "gcc"
cflags := "-O2 -pipe -static"
# Static X11 requires XCB and related libs
libs := "-lm -lX11 -lxcb -lXau -lXdmcp -lpthread -ldl"

default: static

# Build a fully static binary (Linux/X11)
static:
    {{cc}} {{cflags}} noise.c -o noise {{libs}}

run:
    ./noise

clean:
    rm -f noise
