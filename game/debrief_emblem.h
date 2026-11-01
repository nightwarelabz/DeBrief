#ifndef DEBRIEF_EMBLEM_H
#define DEBRIEF_EMBLEM_H

#include "engine.h"
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

#define DEBRIEF_LOGO_H 46

typedef struct {
    char name;
    int w;
    uint64_t rows[DEBRIEF_LOGO_H];
} DeBriefGlyph;

static const DeBriefGlyph DEBRIEF_GLYPHS[7] = {
    { 'D', 38, {
        0x0000003fffffff00ULL, 0x0000003fffffff00ULL, 0x0000003fffffff00ULL, 0x0000003fffffff00ULL,
        0x0000003fffffff00ULL, 0x0000003fffffff00ULL, 0x00000000ffffff80ULL, 0x00000000ffc0ff80ULL,
        0x00000000ffc07fc0ULL, 0x00000000ffc01fe0ULL, 0x00000000ffc01fe0ULL, 0x00000000ffc01fe0ULL,
        0x00000000ffc00ff8ULL, 0x00000000ffc00ff8ULL, 0x00000000ffc00ff8ULL, 0x00000000ffc007fcULL,
        0x00000000ffc007fcULL, 0x00000000ffc007fcULL, 0x00000000ffc003feULL, 0x00000000ffc003feULL,
        0x00000000ffc003feULL, 0x00000000ffc003feULL, 0x00000000ffc003feULL, 0x00000000ffc003feULL,
        0x00000000ffc003feULL, 0x00000000ffc003feULL, 0x00000000ffc003feULL, 0x00000000ffc003feULL,
        0x00000000ffc003feULL, 0x00000000ffc007fcULL, 0x00000000ffc007fcULL, 0x00000000ffc007fcULL,
        0x00000000ffc00ff8ULL, 0x00000000ffc00ff8ULL, 0x00000000ffc00ff8ULL, 0x00000000ffc01fe0ULL,
        0x00000000ffc01fe0ULL, 0x00000000ffc01fe0ULL, 0x000000007fc07fc0ULL, 0x000000003fc0ff80ULL,
        0x000000001fffff80ULL, 0x0000000007ffff00ULL, 0x0000000007ffff00ULL, 0x0000000003ffff00ULL,
        0x0000000001ffff00ULL, 0x0000000000ffff00ULL,
    } },
    { 'e', 26, {
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL,
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL,
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL,
        0x0000000000000000ULL, 0x00000000003ffff8ULL, 0x00000000003ffff8ULL, 0x00000000003ffff8ULL,
        0x00000000007ffff8ULL, 0x00000000007ffff8ULL, 0x00000000007ffff8ULL, 0x00000000007ffff8ULL,
        0x00000000007f8000ULL, 0x00000000007f8000ULL, 0x00000000007f8000ULL, 0x00000000007f8000ULL,
        0x00000000007f8000ULL, 0x00000000007f8000ULL, 0x00000000007f8000ULL, 0x00000000007ffffcULL,
        0x00000000007ffffcULL, 0x00000000007ffffcULL, 0x00000000007ffffcULL, 0x00000000007f8000ULL,
        0x00000000007f8000ULL, 0x00000000007f8000ULL, 0x00000000007f803fULL, 0x00000000007f803fULL,
        0x00000000007f803fULL, 0x00000000007f803fULL, 0x00000000007f803fULL, 0x00000000007f803fULL,
        0x00000000007fffffULL, 0x00000000007ffffcULL, 0x00000000007ffffcULL, 0x00000000007ffffcULL,
        0x00000000003ffffcULL, 0x00000000003ffffcULL,
    } },
    { 'B', 36, {
        0x0000000ffffffe00ULL, 0x0000000ffffffe00ULL, 0x0000000ffffffe00ULL, 0x0000000ffffffe00ULL,
        0x0000000fffffff80ULL, 0x0000000fffffff80ULL, 0x000000003fffffc0ULL, 0x000000003ff00fe0ULL,
        0x000000003ff00fe0ULL, 0x000000003ff007f8ULL, 0x000000003ff007f8ULL, 0x000000003ff007f8ULL,
        0x000000003ff007f8ULL, 0x000000003ff007f8ULL, 0x000000003ff007f8ULL, 0x000000003ff00fe0ULL,
        0x000000003ff00fe0ULL, 0x000000003ff03fc0ULL, 0x000000003ff07f80ULL, 0x000000003ff07f80ULL,
        0x000000003ff0fe00ULL, 0x000000003ffffe00ULL, 0x000000003fffff80ULL, 0x000000003fffffc0ULL,
        0x000000003fffffc0ULL, 0x000000003ff00fe0ULL, 0x000000003ff007f8ULL, 0x000000003ff003fcULL,
        0x000000003ff003fcULL, 0x000000003ff003fcULL, 0x000000003ff001feULL, 0x000000003ff001feULL,
        0x000000003ff001feULL, 0x000000003ff001feULL, 0x000000003ff001feULL, 0x000000003ff001feULL,
        0x000000003ff001feULL, 0x000000003ff001feULL, 0x000000003ff003fcULL, 0x000000003ff003fcULL,
        0x000000003ffffff8ULL, 0x0000000fffffffe0ULL, 0x0000000fffffffe0ULL, 0x0000000fffffffc0ULL,
        0x0000000fffffff80ULL, 0x0000000fffffff80ULL,
    } },
    { 'r', 23, {
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL,
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL,
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL,
        0x0000000000000000ULL, 0x00000000007ffffcULL, 0x00000000007ffffcULL, 0x00000000007ffffcULL,
        0x00000000007ffffcULL, 0x00000000007fffffULL, 0x000000000001ffffULL, 0x000000000001ffffULL,
        0x000000000001ff3fULL, 0x000000000001ff3fULL, 0x000000000001ff3fULL, 0x000000000001ff3fULL,
        0x000000000001ff3fULL, 0x000000000001ff00ULL, 0x000000000001ff00ULL, 0x000000000001ff00ULL,
        0x000000000001ff00ULL, 0x000000000001ff00ULL, 0x000000000001ff00ULL, 0x000000000001ff00ULL,
        0x000000000001ff00ULL, 0x000000000001ff00ULL, 0x000000000001ff00ULL, 0x000000000001ff00ULL,
        0x000000000001ff00ULL, 0x000000000001ff00ULL, 0x000000000001ff00ULL, 0x000000000001ff00ULL,
        0x000000000001ff00ULL, 0x000000000001ff00ULL, 0x000000000001ff00ULL, 0x000000000001ff00ULL,
        0x000000000001ff00ULL, 0x000000000001ff00ULL,
    } },
    { 'i', 18, {
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000fe0ULL,
        0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL,
        0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000000ULL,
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL,
        0x000000000000ffe0ULL, 0x000000000000ffe0ULL, 0x000000000000ffe0ULL, 0x000000000000ffe0ULL,
        0x000000000000ffe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL,
        0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL,
        0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL,
        0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL,
        0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL, 0x0000000000000fe0ULL,
        0x0000000000000fe0ULL, 0x000000000000fffeULL, 0x000000000000fffeULL, 0x000000000000fffeULL,
        0x000000000000fffeULL, 0x000000000000fffeULL,
    } },
    { 'e', 26, {
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL,
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL,
        0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL, 0x0000000000000000ULL,
        0x0000000000000000ULL, 0x00000000003ffff8ULL, 0x00000000003ffff8ULL, 0x00000000003ffff8ULL,
        0x00000000007ffff8ULL, 0x00000000007ffff8ULL, 0x00000000007ffff8ULL, 0x00000000007ffff8ULL,
        0x00000000007f8000ULL, 0x00000000007f8000ULL, 0x00000000007f8000ULL, 0x00000000007f8000ULL,
        0x00000000007f8000ULL, 0x00000000007f8000ULL, 0x00000000007f8000ULL, 0x00000000007ffffcULL,
        0x00000000007ffffcULL, 0x00000000007ffffcULL, 0x00000000007ffffcULL, 0x00000000007f8000ULL,
        0x00000000007f8000ULL, 0x00000000007f8000ULL, 0x00000000007f803fULL, 0x00000000007f803fULL,
        0x00000000007f803fULL, 0x00000000007f803fULL, 0x00000000007f803fULL, 0x00000000007f803fULL,
        0x00000000007fffffULL, 0x00000000007ffffcULL, 0x00000000007ffffcULL, 0x00000000007ffffcULL,
        0x00000000003ffffcULL, 0x00000000003ffffcULL,
    } },
    { 'f', 23, {
        0x0000000000000ffeULL, 0x0000000000000ffeULL, 0x0000000000000ffeULL, 0x0000000000000ffeULL,
        0x0000000000000fffULL, 0x0000000000000fffULL, 0x000000000000ffffULL, 0x000000000000ff1fULL,
        0x000000000000ff1fULL, 0x000000000000ff1fULL, 0x000000000000ff1fULL, 0x000000000000ff00ULL,
        0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL,
        0x00000000001ffffcULL, 0x00000000001ffffcULL, 0x00000000001ffffcULL, 0x00000000001ffffcULL,
        0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL,
        0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL,
        0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL,
        0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL,
        0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL, 0x000000000000ff00ULL,
        0x000000000000ff00ULL, 0x00000000000fff00ULL, 0x00000000000fff00ULL, 0x00000000000fff00ULL,
        0x00000000000fff00ULL, 0x00000000000fff00ULL,
    } },
};


static inline void set_px_fast(FB *fb, int x, int y, uint32_t c) {
    if ((unsigned)x < (unsigned)fb->w && (unsigned)y < (unsigned)fb->h)
        fb->px[y * fb->w + x] = c;
}

static inline uint32_t pack3(int r, int g, int b) {
    if (r < 0) r = 0; else if (r > 255) r = 255;
    if (g < 0) g = 0; else if (g > 255) g = 255;
    if (b < 0) b = 0; else if (b > 255) b = 255;
    return ((uint32_t)r << 16) | ((uint32_t)g << 8) | (uint32_t)b;
}

static inline int lhash(int x, int y, int s) {
    uint32_t v = (uint32_t)x * 374761393u + (uint32_t)y * 668265263u + (uint32_t)s;
    v = (v ^ (v >> 13)) * 1274126177u;
    return (int)((v ^ (v >> 16)) & 0xffu);
}

/* 3D-beveled, brushed-steel procedural DeBrief logo matching the Operator hero portrait */
static void draw_debrief_emblem(FB *fb, int x0, int y0) {
    int cx = x0;

    /* Pass 1: Drop Shadow (+2, +3 offset) */
    for (int g = 0; g < 7; g++) {
        int w = DEBRIEF_GLYPHS[g].w;
        for (int ly = 0; ly < DEBRIEF_LOGO_H; ly++) {
            uint64_t row = DEBRIEF_GLYPHS[g].rows[ly];
            if (!row) continue;
            for (int lx = 0; lx < w; lx++) {
                if ((row >> (w - 1 - lx)) & 1ULL) {
                    set_px_fast(fb, cx + lx + 2, y0 + ly + 3, 0x05070cu);
                    set_px_fast(fb, cx + lx + 3, y0 + ly + 3, 0x05070cu);
                }
            }
        }
        cx += w - 3;
    }

    /* Pass 2: Outer Dark Steel Contour (8-neighbor border) */
    cx = x0;
    for (int g = 0; g < 7; g++) {
        int w = DEBRIEF_GLYPHS[g].w;
        for (int ly = 0; ly < DEBRIEF_LOGO_H; ly++) {
            for (int lx = 0; lx < w; lx++) {
                bool solid = (DEBRIEF_GLYPHS[g].rows[ly] >> (w - 1 - lx)) & 1ULL;
                if (solid) continue;
                bool nbr = false;
                for (int dy = -1; dy <= 1 && !nbr; dy++) {
                    int ny = ly + dy;
                    if (ny < 0 || ny >= DEBRIEF_LOGO_H) continue;
                    uint64_t nrow = DEBRIEF_GLYPHS[g].rows[ny];
                    for (int dx = -1; dx <= 1; dx++) {
                        int nx = lx + dx;
                        if (nx >= 0 && nx < w && ((nrow >> (w - 1 - nx)) & 1ULL)) {
                            nbr = true;
                            break;
                        }
                    }
                }
                if (nbr)
                    set_px_fast(fb, cx + lx, y0 + ly, 0x141820u);
            }
        }
        cx += w - 3;
    }

    /* Pass 3: 3D Bevel Highlights/Shadows + Brushed Metal Texture */
    cx = x0;
    for (int g = 0; g < 7; g++) {
        int w = DEBRIEF_GLYPHS[g].w;
        for (int ly = 0; ly < DEBRIEF_LOGO_H; ly++) {
            uint64_t row = DEBRIEF_GLYPHS[g].rows[ly];
            if (!row) continue;
            uint64_t above = (ly > 0) ? DEBRIEF_GLYPHS[g].rows[ly - 1] : 0ULL;
            uint64_t below = (ly < DEBRIEF_LOGO_H - 1) ? DEBRIEF_GLYPHS[g].rows[ly + 1] : 0ULL;

            for (int lx = 0; lx < w; lx++) {
                if (!((row >> (w - 1 - lx)) & 1ULL)) continue;

                bool left_e  = (lx == 0) || !((row >> (w - 1 - (lx - 1))) & 1ULL);
                bool top_e   = (ly == 0) || !((above >> (w - 1 - lx)) & 1ULL);
                bool right_e = (lx == w - 1) || !((row >> (w - 1 - (lx + 1))) & 1ULL);
                bool bot_e   = (ly == DEBRIEF_LOGO_H - 1) || !((below >> (w - 1 - lx)) & 1ULL);

                uint32_t col;
                if (top_e && left_e) {
                    col = 0xfcfdffu;
                } else if (top_e) {
                    col = 0xd8e2eeu;
                } else if (left_e) {
                    col = 0xcad4e2u;
                } else if (bot_e && right_e) {
                    col = 0x222630u;
                } else if (bot_e) {
                    col = 0x303642u;
                } else if (right_e) {
                    col = 0x3e4554u;
                } else {
                    int grain = (lhash(lx + g * 37, ly, 103) % 20) - 10;
                    int scratch = ((ly * 13) % 7 == 0) ? -9 : 4;
                    int base = 196 - (ly * 44) / DEBRIEF_LOGO_H;
                    int v = base + grain + scratch;
                    if (v < 70) v = 70; else if (v > 240) v = 240;
                    col = pack3(v - 2, v, v + 6);
                }
                set_px_fast(fb, cx + lx, y0 + ly, col);
            }
        }
        cx += w - 3;
    }

    int total_w = cx - x0;
    int rule_y = y0 + DEBRIEF_LOGO_H + 7;
    int mid_x = x0 + total_w / 2;

    /* Tapered crimson horizontal rule */
    for (int x = x0 - 4; x <= x0 + total_w + 8; x++) {
        int d = abs(x - mid_x);
        int max_d = total_w / 2 + 10;
        float a = 1.0f - (float)d / (float)max_d;
        if (a < 0.0f) a = 0.0f;
        a = a * a;
        int r = (int)(195.0f * a);
        int g = (int)(28.0f * a);
        int b = (int)(38.0f * a);
        set_px_fast(fb, x, rule_y, pack3(r, g, b));
        set_px_fast(fb, x, rule_y + 1, pack3(r * 2 / 3, g * 2 / 3, b * 2 / 3));
    }

    /* Glowing ruby square positioned exactly under B */
    int sq_x = x0 + (DEBRIEF_GLYPHS[0].w - 3) + (DEBRIEF_GLYPHS[1].w - 3) + DEBRIEF_GLYPHS[2].w / 2;
    for (int dy = -4; dy <= 4; dy++) {
        for (int dx = -4; dx <= 4; dx++) {
            if (abs(dx) == 4 || abs(dy) == 4) {
                set_px_fast(fb, sq_x + dx, rule_y + dy, 0x48080cu);
            } else if (abs(dx) <= 2 && abs(dy) <= 2) {
                set_px_fast(fb, sq_x + dx, rule_y + dy, 0xff343eu);
            } else {
                set_px_fast(fb, sq_x + dx, rule_y + dy, 0xda1c26u);
            }
        }
    }

    /* Subtitle: "A   c i t y   o f   s p i e s" */
    raster_text(fb, x0 + 31, rule_y + 11, "A   c i t y   o f   s p i e s", 0x06080eu);
    raster_text(fb, x0 + 30, rule_y + 10, "A   c i t y   o f   s p i e s", 0xe6edf8u);
}

/* Bottom-left rating badge and 3D C35 engine mark */
static void draw_debrief_footer(FB *fb) {
    int rx = 22, ry = fb->h - 42;

    /* Rating Box (26x32) */
    raster_rect(fb, rx, ry, 26, 32, 0xffffffu);
    raster_rect(fb, rx + 1, ry + 1, 24, 30, 0x080a0eu);
    raster_rect(fb, rx + 1, ry + 8, 24, 1, 0xffffffu);

    /* "RATING" at top */
    raster_text(fb, rx + 1, ry + 2, "RATING", 0xffffffu);

    /* Bold centered "N/A" */
    static const uint8_t NA_GLYPHS[3][10] = {
        { 0x33, 0x33, 0x3b, 0x3b, 0x37, 0x37, 0x33, 0x33, 0x33, 0x33 },
        { 0x01, 0x01, 0x02, 0x02, 0x04, 0x04, 0x08, 0x08, 0x08, 0x00 },
        { 0x0c, 0x1e, 0x33, 0x33, 0x3f, 0x3f, 0x33, 0x33, 0x33, 0x33 }
    };
    int na_x = rx + 3, na_y = ry + 12;
    for (int ly = 0; ly < 10; ly++) {
        uint8_t r0 = NA_GLYPHS[0][ly];
        uint8_t r1 = NA_GLYPHS[1][ly];
        uint8_t r2 = NA_GLYPHS[2][ly];
        for (int lx = 0; lx < 6; lx++)
            if ((r0 >> (5 - lx)) & 1) set_px_fast(fb, na_x + lx, na_y + ly, 0xffffffu);
        for (int lx = 0; lx < 4; lx++)
            if ((r1 >> (3 - lx)) & 1) set_px_fast(fb, na_x + 8 + lx, na_y + ly, 0xd0d8e8u);
        for (int lx = 0; lx < 6; lx++)
            if ((r2 >> (5 - lx)) & 1) set_px_fast(fb, na_x + 14 + lx, na_y + ly, 0xffffffu);
    }

    /* "Made with the CASM-P15 engine" */
    int tx = rx + 34;
    raster_text(fb, tx, ry + 14, "Made with the ", 0xc4d0e0u);

    /* Iconic retro-console brand colors for CASM-P15 */
    static const uint32_t CASM_COLS[8] = {
        0x00b4d8u, /* C: Cyan */
        0xffb703u, /* A: Amber Gold */
        0x52b788u, /* S: Emerald Mint */
        0xff4d6du, /* M: Crimson Ruby */
        0xd0dae8u, /* -: Brushed Steel */
        0x38bdf8u, /* P: Electric Sky */
        0xffb703u, /* 1: Amber Gold */
        0x52b788u  /* 5: Emerald Mint */
    };
    static const char *CASM_STR[8] = { "C", "A", "S", "M", "-", "P", "1", "5" };
    int bx = tx + 14 * 6;
    for (int k = 0; k < 8; k++) {
        raster_text(fb, bx + k * 6, ry + 14, CASM_STR[k], CASM_COLS[k]);
    }

    raster_text(fb, bx + 8 * 6, ry + 14, " engine", 0xc4d0e0u);
}

#endif /* DEBRIEF_EMBLEM_H */
