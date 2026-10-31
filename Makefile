CC      ?= cc
CFLAGS  ?= -O2 -std=c11 -Wall -Wextra -Werror -march=native -Iengine/src
UNAME_S := $(shell uname -s)
ARCH    := $(shell uname -m)

CORE     = engine/src/raster.c engine/src/render.c engine/src/texgen.c \
           engine/src/mesh.c engine/src/assets.c engine/src/anim.c \
           engine/src/physics.c engine/src/audio.c engine/src/percept.c \
           engine/src/persist.c

ifeq ($(ARCH),x86_64)
CORE    += engine/src/fill_x86_64.s
CFLAGS  += -DCASM_HAVE_ASM_FILL=1
endif
ifeq ($(ARCH),arm64)
CORE    += engine/src/fill_arm64.S
CFLAGS  += -DCASM_HAVE_ASM_FILL=1
endif
ifeq ($(ARCH),aarch64)
CORE    += engine/src/fill_arm64.S
CFLAGS  += -DCASM_HAVE_ASM_FILL=1
endif

ifeq ($(UNAME_S),Linux)
PLAT      = engine/src/platform_x11.c engine/src/audio_device_linux.c
PLAT_LIBS = -lX11 -lasound
endif
ifeq ($(UNAME_S),Darwin)
PLAT      = engine/src/platform_macos.m engine/src/audio_device_macos.c
PLAT_LIBS = -framework Cocoa -framework AudioToolbox
endif

debrief: game/debrief.c $(CORE) $(PLAT) engine/src/engine.h engine/src/platform.h
	$(CC) $(CFLAGS) -o $@ game/debrief.c $(CORE) $(PLAT) -lm $(PLAT_LIBS)

clean:
	rm -f debrief

.PHONY: clean
