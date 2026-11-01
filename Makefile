CC      ?= cc
CFLAGS  ?= -O2 -std=c11 -Wall -Wextra -Werror -march=native -Iengine/src
UNAME_S := $(shell uname -s)
ARCH    := $(shell uname -m)

CORE     = engine/src/raster.c engine/src/render.c engine/src/texgen.c \
           engine/src/mesh.c engine/src/assets.c engine/src/anim.c \
           engine/src/physics.c engine/src/audio.c engine/src/percept.c \
           engine/src/persist.c engine/src/greyport.c

# The fill kernels use C-preprocessor macros (SYM), so they must be
# assembled as their own translation units with -x assembler-with-cpp —
# feeding the .s/.S straight into the link line hands it to the assembler
# unpreprocessed and every SYM() line fails. Same shape as CASM-PS15's
# own Makefile pattern rules.
ASM_OBJ  =
ifeq ($(ARCH),x86_64)
ASM_OBJ  = engine/src/fill_x86_64.o
CFLAGS  += -DCASM_HAVE_ASM_FILL=1
endif
ifeq ($(ARCH),arm64)
ASM_OBJ  = engine/src/fill_arm64.o
CFLAGS  += -DCASM_HAVE_ASM_FILL=1
endif
ifeq ($(ARCH),aarch64)
ASM_OBJ  = engine/src/fill_arm64.o
CFLAGS  += -DCASM_HAVE_ASM_FILL=1
endif

ifeq ($(UNAME_S),Linux)
PLAT      = engine/src/platform_x11.c engine/src/audio_device_linux.c
PLAT_LIBS = -lX11 -lasound -lpthread
endif
ifeq ($(UNAME_S),Darwin)
PLAT      = engine/src/platform_macos.m engine/src/audio_device_macos.c
PLAT_LIBS = -framework Cocoa -framework AudioToolbox
endif

HEADERS = engine/src/engine.h engine/src/greyport.h engine/src/platform.h game/debrief_emblem.h

all: debrief-game

debrief-game: game/debrief.c $(CORE) $(PLAT) $(ASM_OBJ) $(HEADERS)
	$(CC) $(CFLAGS) -o $@ game/debrief.c $(CORE) $(PLAT) $(ASM_OBJ) -lm $(PLAT_LIBS)

engine/src/fill_x86_64.o: engine/src/fill_x86_64.s
	$(CC) $(CFLAGS) -x assembler-with-cpp -c $< -o $@

engine/src/fill_arm64.o: engine/src/fill_arm64.S
	$(CC) $(CFLAGS) -x assembler-with-cpp -c $< -o $@

clean:
	rm -f debrief-game debrief_save.dat $(ASM_OBJ)

.PHONY: all clean
