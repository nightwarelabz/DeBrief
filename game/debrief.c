/* DeBrief: A City of Spies — Playable Third-Person Espionage Action Game
 * Built on the zero-dependency CASM-PS15 C11 + SIMD Assembly Engine.
 *
 * Visual Doctrine: PlayStation 1.5 cold-war nocturne faithful to the Operator
 * banner portrait (pixel-camo hood, exposed sunlit/sodium brow, ruby mosaic
 * visor + LED, gold NSA crest, twin unequal radio whips, brushed gunmetal USP,
 * flooded harbor canal planar reflections, and Sony Trinitron scanlines).
 *
 * Gameplay Doctrine: 100% modern third-person stealth-action systems:
 *   - Seamless live-3D Title Banner -> Over-the-Shoulder camera handoff
 *   - WASD + Mouse 360 camera-relative movement, RMB tight right-shoulder ADS
 *   - LMB suppressed HK USP (headshots, shootable sodium lamps, wall-ping
 *     acoustic lures, Patrol Mecha SENSOR / ACTUATOR / COOLANT surgery)
 *   - E physical raycast-anything grapple (hold C/Ctrl to lock rope length & swing)
 *   - Space coyote/buffered jump, sprint crouch-slide, kinetic wall-run &
 *     wall-jump, automatic waist/chest ledge vaulting
 *   - F silent CQC takedown + don/shed Regime Trooper Greatcoat Disguise
 *   - R hack classified Archive Terminals / reload HK USP / launch new Op
 *   - N toggle Crimson SIGINT Night-Optic Visor, M toggle Handler Dossier */
#define _POSIX_C_SOURCE 200809L
#include "engine.h"
#include "greyport.h"
#include "platform.h"
#include "debrief_emblem.h"
#include <time.h>

static const V3 GUARD0_WP[] = {
    { 7.5f, 0.0f, 21.5f }, { 2.6f, 0.0f, 25.0f },
    { 2.6f, 0.0f, 14.5f }, { 7.5f, 0.0f, 15.0f },
};
static const V3 GUARD1_WP[] = {
    { -2.4f, 0.0f, 29.5f }, {  1.8f, 0.0f, 30.5f },
    {  1.8f, 0.0f, 26.8f }, { -2.4f, 0.0f, 27.2f },
};
static const V3 MECHA_WP[] = {
    { 9.2f, 0.0f, 11.5f }, { 9.2f, 0.0f, 21.6f },
    { 6.8f, 0.0f, 21.6f }, { 6.8f, 0.0f, 11.5f },
};

typedef struct {
    V3 pos;
    int value;
    uint32_t col;
    float timer;
    const char *tag;
} FloatPop;

static double now_s(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec + ts.tv_nsec / 1e9;
}

static float turn_toward(float cur, float target, float max_step) {
    float d = target - cur;
    while (d >  PS15_PI) d -= 2.0f * PS15_PI;
    while (d < -PS15_PI) d += 2.0f * PS15_PI;
    if (fabsf(d) <= max_step) return target;
    return cur + (d > 0.0f ? max_step : -max_step);
}

static uint32_t present_buf[960 * 540];
static void present(Plat *plat, const FB *fb) {
    if (fb->settings.crt_scanlines)
        ps15_upscale2x_crt(present_buf, fb->px, fb->w, fb->h);
    else
        ps15_upscale2x(present_buf, fb->px, fb->w, fb->h);
    plat_present(plat, present_buf);
}

static int dump_ppm(const char *path, const FB *fb) {
    uint32_t *up = fb->settings.crt_scanlines ? fb_upscale2x_crt(fb) : fb_upscale2x(fb);
    FILE *f = fopen(path, "wb");
    if (!f) { free(up); return -1; }
    fprintf(f, "P6\n%d %d\n255\n", fb->w * 2, fb->h * 2);
    size_t npix = (size_t)(fb->w * 2) * (size_t)(fb->h * 2);
    unsigned char *rgb = malloc(npix * 3);
    for (size_t i = 0; i < npix; i++) {
        rgb[i * 3 + 0] = (unsigned char)(up[i] >> 16);
        rgb[i * 3 + 1] = (unsigned char)(up[i] >> 8);
        rgb[i * 3 + 2] = (unsigned char)(up[i]);
    }
    size_t wr = fwrite(rgb, 1, npix * 3, f);
    free(rgb);
    fclose(f);
    free(up);
    return (wr == npix * 3) ? 0 : -1;
}

static void spawn_pop(FloatPop pops[8], V3 pos, int val, const char *tag, uint32_t col) {
    int slot = 0;
    float best = 999.0f;
    for (int i = 0; i < 8; i++) {
        if (pops[i].timer <= 0.0f) { slot = i; break; }
        if (pops[i].timer < best) { best = pops[i].timer; slot = i; }
    }
    pops[slot] = (FloatPop){ pos, val, col, 1.20f, tag };
}

/* debrief_emblem.h provides the high-res 3D-beveled brushed-steel DeBrief logo & footer */

static void draw_ping_ring(FB *fb, const Camera *cam, V3 center, float radius, uint32_t col) {
    const int N = 12;
    for (int i = 0; i < N; i++) {
        float a0 = (float)i * (2.0f * PS15_PI / (float)N);
        float a1 = (float)(i + 1) * (2.0f * PS15_PI / (float)N);
        V3 p0 = v3(center.x + cosf(a0) * radius, center.y + 0.08f, center.z + sinf(a0) * radius);
        V3 p1 = v3(center.x + cosf(a1) * radius, center.y + 0.08f, center.z + sinf(a1) * radius);
        render_line3d(fb, cam, p0, p1, col, BLEND_ADD);
    }
}

static const char *compute_grade(int alarms, int takedowns, int shots_fired, bool mecha_down) {
    if (alarms == 0 && takedowns == 0 && !mecha_down && shots_fired == 0)
        return "GHOST-THAT-NEVER-WAS";
    if (alarms == 0 && takedowns <= 1 && !mecha_down)
        return "GHOST";
    if (alarms == 0)
        return "SURGEON";
    if (alarms <= 2 && !mecha_down)
        return "OPERATIVE";
    return "BUTCHER";
}

static const char *handler_remark(int alarms, int takedowns, bool mecha_down, bool overwatch_out) {
    if (alarms == 0 && takedowns == 0 && !mecha_down)
        return "ZERO TRACE IN GREYPORT. REGIME STILL DOES NOT KNOW WE WERE HERE.";
    if (alarms == 0 && overwatch_out)
        return "CLEAN EXTRACTION UNDER HARBOR BLACKOUT. ARCHIVE FILED INTACT.";
    if (mecha_down)
        return "PATROL WALKER WRECKED ON THE QUAY. EXPECT GARRISON LOCKDOWN.";
    if (alarms == 0)
        return "SILENT SURGERY ON THE PIER. NO SIRENS RAISED.";
    return "COMPROMISED ON THE DOCKS, BUT NIGHTJAR FILES MADE THE BARGE.";
}

int main(int argc, char **argv) {
    long max_frames = 0;
    bool skip_title = false;
    const char *dump_path = NULL;
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--frames") == 0 && i + 1 < argc)
            max_frames = strtol(argv[++i], NULL, 10);
        else if (strcmp(argv[i], "--gameplay") == 0)
            skip_title = true;
        else if (strcmp(argv[i], "--dump-ppm") == 0 && i + 1 < argc)
            dump_path = argv[++i];
    }

    GreyportScene sc;
    greyport_init(&sc);
    FB *fb = fb_new(FB_W, FB_H);
    Humanoid oper    = humanoid_new();
    Humanoid soldier = humanoid_guard_new();
    MechaMesh mmesh  = mecha_mesh_new();

    Plat *plat = plat_open("DeBrief — A City of Spies", 960, 540);
    if (!plat) { fprintf(stderr, "no window available\n"); return 1; }
    AudioLive live;
    au_live_init(&live);
    int audio_ok = (audev_start(&live) == 0);
    printf("window: X11/Cocoa platform  audio: %s\n",
           audio_ok ? audev_name() : "unavailable (silent session)");

    /* Load prior career best alert if debrief_save.dat exists */
    SaveState prior_sv;
    bool have_prior = (save_read("debrief_save.dat", &prior_sv) == 0);
    float best_alert_record = have_prior ? prior_sv.alert : -1.0f;

    /* Spawn Operator at the exact West Quay banner vantage so Title -> Gameplay is seamless! */
    Body player = { v3(2.15f, 0.00f, 12.52f), v3(0, 0, 0), 0.35f, 1.8f, true, false, v3(0, 0, 0), 0 };
    Guard guards[2] = {
        { v3( 5.4f, 0.0f, 19.4f), -2.55f, 0.0f, 0, 0.0f, 0.0f, v3(0,0,0), 0.0f },
        { v3(-2.0f, 0.0f, 28.0f),  1.80f, 0.0f, 0, 0.4f, 0.0f, v3(0,0,0), 0.0f }
    };
    Mecha walker;
    mecha_init(&walker, v3(3.8f, 0.0f, 25.5f), -2.72f);

    bool title_active = !skip_title;
    float title_blend = skip_title ? 0.0f : 1.0f;
    float cam_yaw = -0.06f, cam_pitch = 0.02f;
    float char_yaw = skip_title ? 0.20f : 3.32f;
    Camera cam = tps_camera_step((Camera){ player.pos, cam_yaw, cam_pitch },
                                 &player, cam_yaw, cam_pitch, false, false,
                                 sc.world, sc.nworld, 1.0f);

    int showcase_front = 0, show_tech = 0, show_dossier = 0;
    int ammo = 12, shots_fired = 0, takedowns = 0, alarms = 0, grapples = 0;
    bool crouched = false, alarmed = false, grapple_latch_guard = false;
    bool disguised = false, mecha_scanned = false, extracted = false;
    float slide_timer = 0.0f, coyote_t = 0.0f, jump_buf_t = 0.0f, wall_timer = 1.35f;
    V3 slide_dir = v3(0, 0, 1), wall_norm = v3(0, 0, 0);
    bool wall_running = false;
    float fire_cd = 0.0f, recoil = 0.0f, hitmark_t = 0.0f, op_time = 0.0f;
    float alert_peak = 0.0f, step_acc = 0.0f, noise = 0.0f, last_render_ms = 0.0f;
    V3 ping_pos = v3(0, 0, 0);
    float ping_timer = 0.0f;
    const char *banner_msg = "OP NIGHTJAR: HACK 2 TERMINALS [R] + EXTRACT WEST BARGE";
    float banner_t = skip_title ? 5.0f : 0.0f;
    FloatPop pops[8];
    memset(pops, 0, sizeof pops);

    long frames = 0;
    double start = now_s(), last = start, acc = 0.0;
    const float DT = 1.0f / 60.0f;

    Input in;
    while (plat_poll(plat, &in)) {
        double tnow = now_s();
        double ft = tnow - last;
        last = tnow;
        if (ft > 0.25) ft = 0.25;
        acc += ft;

        if (in.keys & K_QUIT) break;

        /* Deploy from live 3D Title Banner into Third-Person Over-the-Shoulder */
        if (title_active) {
            if (in.pressed & (K_JUMP | K_FIRE | K_USE | K_GRAPPLE)) {
                title_active = false;
                in.pressed &= ~(K_JUMP | K_FIRE | K_USE | K_GRAPPLE);
                au_live_visor(&live);
                banner_t = 5.0f;
            }
        }

        bool aiming = !title_active && ((in.keys & K_AIM) != 0);
        if (!title_active) {
            float sens = aiming ? 0.0017f : 0.0028f;
            cam_yaw += in.mouse_dx * sens;
            cam_pitch = clampf(cam_pitch - in.mouse_dy * sens, -1.18f, 1.15f);
            if (in.keys & K_LOOK_L) cam_yaw   -= (float)ft * 2.1f;
            if (in.keys & K_LOOK_R) cam_yaw   += (float)ft * 2.1f;
            if (in.keys & K_LOOK_U) cam_pitch  = clampf(cam_pitch + (float)ft * 1.5f, -1.18f, 1.15f);
            if (in.keys & K_LOOK_D) cam_pitch  = clampf(cam_pitch - (float)ft * 1.5f, -1.18f, 1.15f);

            if (in.pressed & K_TOGGLE_VIEW) showcase_front = !showcase_front;
            if (in.pressed & K_DOSSIER)     show_dossier = !show_dossier;
            if (in.pressed & K_JUMP)        jump_buf_t = 0.14f;
        }
        if (in.pressed & K_HUD)         show_tech = !show_tech;
        if (in.pressed & K_SNAP_MODE)   fb->settings.snap_mode = (fb->settings.snap_mode + 1) % 3;
        if (in.pressed & K_UV_MODE)     fb->settings.affine_uv = !fb->settings.affine_uv;
        if (in.pressed & K_DAC_MODE) {
            GraphicProfile next = (fb->settings.profile + 1) % 3;
            fb_set_profile(fb, next);
        }
        if (in.pressed & K_WIRE_MODE)   fb->settings.wireframe = !fb->settings.wireframe;
        if (in.pressed & K_CRT)         fb->settings.crt_scanlines = !fb->settings.crt_scanlines;
        if (in.pressed & K_VISOR) {
            fb->settings.visor_mode = !fb->settings.visor_mode;
            au_live_visor(&live);
        }

        while (acc >= DT) {
            acc -= DT;
            if (!title_active) {
                title_blend = fmaxf(0.0f, title_blend - DT * 1.85f);
                if (!extracted) op_time += DT;
            }
            if (fire_cd > 0.0f)    fire_cd    = fmaxf(0.0f, fire_cd - DT);
            if (recoil > 0.0f)     recoil     = fmaxf(0.0f, recoil - DT * 5.5f);
            if (hitmark_t > 0.0f)  hitmark_t  = fmaxf(0.0f, hitmark_t - DT);
            if (banner_t > 0.0f)   banner_t   = fmaxf(0.0f, banner_t - DT);
            if (ping_timer > 0.0f) ping_timer = fmaxf(0.0f, ping_timer - DT);
            if (sc.muzzle_flash > 0.0f) sc.muzzle_flash = fmaxf(0.0f, sc.muzzle_flash - DT * 9.0f);
            if (jump_buf_t > 0.0f) jump_buf_t = fmaxf(0.0f, jump_buf_t - DT);
            for (int i = 0; i < 8; i++)
                if (pops[i].timer > 0.0f) pops[i].timer = fmaxf(0.0f, pops[i].timer - DT);

            if (title_active) {
                char_yaw = 3.32f;
                /* Subtle patrol life in the background harbor while on Title Banner */
                guard_update_ex(&guards[0], GUARD0_WP, 4, v3(0, -50, 0), 0.0f, 0.0f, sc.world, sc.nworld, DT * 0.4f);
                mecha_update(&walker, MECHA_WP, 4, v3(0, -50, 0), 0.0f, 0.0f, sc.world, sc.nworld, DT * 0.35f);
                continue;
            }

            V3 fwd = v3(sinf(cam_yaw), 0.0f, cosf(cam_yaw));
            V3 right = v3(fwd.z, 0.0f, -fwd.x);
            V3 wish = v3(0, 0, 0);
            if (in.keys & K_FWD)   wish = v3_add(wish, fwd);
            if (in.keys & K_BACK)  wish = v3_sub(wish, fwd);
            if (in.keys & K_RIGHT) wish = v3_add(wish, right);
            if (in.keys & K_LEFT)  wish = v3_sub(wish, right);
            bool moving = v3_len(wish) > 0.1f;
            if (moving) wish = v3_norm(wish);

            /* Sprint -> Tactical Crouch-Slide into cover */
            bool want_crouch = (in.keys & K_CROUCH) != 0;
            float cur_hspd = hypotf(player.vel.x, player.vel.z);
            if ((in.pressed & K_CROUCH) && player.grounded && cur_hspd > 4.6f && slide_timer <= 0.0f) {
                slide_timer = 0.52f;
                slide_dir = moving ? wish : v3(sinf(char_yaw), 0.0f, cosf(char_yaw));
                crouched = true;
                player.height = 1.15f;
                au_live_footstep(&live, 1);
                in.pressed &= ~K_CROUCH;
            } else if (want_crouch || slide_timer > 0.0f) {
                crouched = true;
                player.height = 1.2f;
            } else if (crouched && physics_can_stand(&player, sc.world, sc.nworld, 1.8f)) {
                crouched = false;
                player.height = 1.8f;
            }

            if (slide_timer > 0.0f) {
                slide_timer = fmaxf(0.0f, slide_timer - DT);
                float s_spd = 7.4f * (0.45f + 0.55f * (slide_timer / 0.52f));
                player.vel.x = slide_dir.x * s_spd;
                player.vel.z = slide_dir.z * s_spd;
            } else if (!player.grappling) {
                float target_spd = aiming ? 2.15f : crouched ? 1.65f :
                                   (in.keys & K_SPRINT) ? 6.1f : 3.65f;
                float tx = moving ? wish.x * target_spd : 0.0f;
                float tz = moving ? wish.z * target_spd : 0.0f;
                float accel = player.grounded ? 18.0f : 6.5f;
                player.vel.x = lerpf(player.vel.x, tx, clampf(accel * DT, 0.0f, 1.0f));
                player.vel.z = lerpf(player.vel.z, tz, clampf(accel * DT, 0.0f, 1.0f));
            } else if (moving) {
                player.vel.x += wish.x * 7.8f * DT;
                player.vel.z += wish.z * 7.8f * DT;
            }

            if (aiming || recoil > 0.05f)
                char_yaw = turn_toward(char_yaw, cam_yaw, 18.0f * DT);
            else if (slide_timer > 0.0f)
                char_yaw = turn_toward(char_yaw, atan2f(slide_dir.x, slide_dir.z), 14.0f * DT);
            else if (moving)
                char_yaw = turn_toward(char_yaw, atan2f(wish.x, wish.z), 12.5f * DT);

            /* Coyote jump + Jump buffer + Wall-jump */
            if (player.grounded) coyote_t = 0.12f;
            else                 coyote_t = fmaxf(0.0f, coyote_t - DT);

            if (jump_buf_t > 0.0f) {
                if (coyote_t > 0.0f) {
                    player.vel.y = 8.7f;
                    player.grounded = false;
                    coyote_t = 0.0f;
                    jump_buf_t = 0.0f;
                } else if (wall_running) {
                    player.vel.y = 8.4f;
                    player.vel.x += wall_norm.x * 5.6f;
                    player.vel.z += wall_norm.z * 5.6f;
                    wall_running = false;
                    jump_buf_t = 0.0f;
                    au_live_footstep(&live, 0);
                }
            }

            /* Physical Raycast-Anything Grapple (Crouch holds rope length for pure pendulum!) */
            if (!(in.keys & K_GRAPPLE)) grapple_latch_guard = false;
            if ((in.keys & K_GRAPPLE) && !player.grappling && !grapple_latch_guard) {
                V3 aim_dir = v3(sinf(cam_yaw) * cosf(cam_pitch), sinf(cam_pitch),
                                cosf(cam_yaw) * cosf(cam_pitch));
                V3 launch_eye = v3(player.pos.x, player.pos.y + 1.52f, player.pos.z);
                if (grapple_fire_ray(&player, launch_eye, aim_dir,
                                     sc.world, sc.nworld, sc.anchors, sc.nanchors, 32.0f)) {
                    au_live_grapple(&live);
                    grapples++;
                    noise = 0.78f;
                    disguised = false; /* grappling sheds greatcoat cover */
                }
                grapple_latch_guard = true;
            }
            if (player.grappling && !(in.keys & K_GRAPPLE))
                grapple_release(&player);

            /* Suppressed HK USP Fire (LMB) */
            if ((in.pressed & K_FIRE) && fire_cd <= 0.0f && !player.grappling) {
                in.pressed &= ~K_FIRE;
                if (ammo > 0) {
                    ammo--;
                    shots_fired++;
                    fire_cd = 0.17f;
                    recoil = 1.0f;
                    char_yaw = cam_yaw;
                    disguised = false;
                    au_live_pistol(&live);
                    sc.muzzle_pos = v3(player.pos.x + sinf(char_yaw) * 0.55f,
                                       player.pos.y + (crouched ? 1.05f : 1.42f),
                                       player.pos.z + cosf(char_yaw) * 0.55f);
                    sc.muzzle_flash = 1.0f;
                    noise = fmaxf(noise, 0.45f);

                    V3 rdir = v3_norm(v3(sinf(cam.yaw) * cosf(cam.pitch), sinf(cam.pitch),
                                         cosf(cam.yaw) * cosf(cam.pitch)));
                    V3 rorig = v3_add(cam.pos, v3_scale(rdir, 1.15f));
                    V3 hit_pt;
                    int hk = greyport_fire_ray_ex(&sc, rorig, rdir, guards, 2, &walker, &hit_pt);
                    if (hk == GP_HIT_WALL) {
                        ping_pos = hit_pt;
                        ping_timer = 1.1f;
                        spawn_pop(pops, hit_pt, 0, "LURE", 0x94c8ffu);
                    } else if (hk == GP_HIT_LAMP) {
                        au_live_glass(&live);
                        hitmark_t = 0.22f;
                        ping_pos = hit_pt;
                        ping_timer = 1.1f;
                        spawn_pop(pops, hit_pt, 0, "LAMP OUT", 0xffd258u);
                    } else if (hk == GP_HIT_GUARD_HEAD) {
                        takedowns++;
                        hitmark_t = 0.28f;
                        spawn_pop(pops, hit_pt, 100, "CRIT", 0xff6238u);
                    } else if (hk == GP_HIT_GUARD_BODY) {
                        takedowns++;
                        hitmark_t = 0.20f;
                        spawn_pop(pops, hit_pt, 38, NULL, 0xffbe3cu);
                    } else if (hk == GP_HIT_MECHA_SENSOR) {
                        hitmark_t = 0.25f;
                        mecha_scanned = true;
                        spawn_pop(pops, hit_pt, 45, "SENSOR", 0xff6238u);
                    } else if (hk == GP_HIT_MECHA_ACTUATOR) {
                        hitmark_t = 0.22f;
                        mecha_scanned = true;
                        spawn_pop(pops, hit_pt, 32, "ACTUATOR", 0xffbe3cu);
                    } else if (hk == GP_HIT_MECHA_COOLANT) {
                        hitmark_t = 0.30f;
                        mecha_scanned = true;
                        spawn_pop(pops, hit_pt, 85, walker.coolant_hp <= 0.01f ? "MECHA DOWN" : "COOLANT", 0xff4838u);
                    }
                } else {
                    au_live_tone(&live, 0.04f, 420.0f, 310.0f, 0.08f);
                    banner_msg = "MAGAZINE EMPTY - PRESS [R] TO RELOAD";
                    banner_t = 2.0f;
                }
            }

            /* Silent CQC Takedown OR Field Greatcoat Disguise (F) */
            if (in.pressed & K_MELEE) {
                in.pressed &= ~K_MELEE;
                bool acted = false;
                for (int i = 0; i < 2; i++) {
                    if (guards[i].stunned > 0.0f) continue;
                    if (v3_len(v3_sub(guards[i].pos, player.pos)) < 2.15f) {
                        guards[i].stunned = 40.0f;
                        guards[i].alert = 0.0f;
                        takedowns++;
                        au_live_footstep(&live, 0);
                        spawn_pop(pops, v3_add(guards[i].pos, v3(0, 1.5f, 0)), 0, "TAKEDOWN", 0x74e8a8u);
                        acted = true;
                        break;
                    }
                }
                if (!acted) {
                    for (int i = 0; i < 2; i++) {
                        if (guards[i].stunned > 0.0f &&
                            v3_len(v3_sub(guards[i].pos, player.pos)) < 2.25f) {
                            disguised = !disguised;
                            au_live_footstep(&live, 0);
                            spawn_pop(pops, v3_add(player.pos, v3(0, 1.65f, 0)), 0,
                                      disguised ? "DISGUISE ON" : "DISGUISE OFF", 0xffd468u);
                            banner_msg = disguised
                                ? "TROOPER GREATCOAT DONNED - WALK CALMLY (MECHA READS BIOMETRICS)"
                                : "FIELD DISGUISE SHED";
                            banner_t = 3.5f;
                            break;
                        }
                    }
                }
            }

            /* Interact / Hack Archive Terminal / Reload / Launch New Op (R) */
            if (in.pressed & K_USE) {
                in.pressed &= ~K_USE;
                if (extracted) {
                    player.pos = v3(-1.45f, 0.0f, 9.20f);
                    player.vel = v3(0, 0, 0);
                    sc.term_hacked[0] = sc.term_hacked[1] = false;
                    sc.lamp_out[0] = sc.lamp_out[1] = sc.lamp_out[2] = false;
                    guards[0] = (Guard){ v3( 6.4f, 0.0f, 20.5f), -2.65f, 0.0f, 0, 0.0f, 0.0f, v3(0,0,0), 0.0f };
                    guards[1] = (Guard){ v3(-2.4f, 0.0f, 29.5f),  1.57f, 0.0f, 0, 0.4f, 0.0f, v3(0,0,0), 0.0f };
                    mecha_init(&walker, v3(8.6f, 0.0f, 19.8f), -2.75f);
                    ammo = 12; shots_fired = 0; takedowns = 0; alarms = 0; op_time = 0.0f;
                    alert_peak = 0.0f; disguised = false; mecha_scanned = false;
                    extracted = false; show_dossier = 0;
                    banner_msg = "NEW OPERATION LIVE - INFILTRATE GREYPORT DOCKYARD";
                    banner_t = 3.5f;
                } else {
                    bool hacked_any = false;
                    for (int i = 0; i < GREYPORT_NUM_TERMS; i++) {
                        if (!sc.term_hacked[i] &&
                            v3_len(v3_sub(sc.term_pos[i], player.pos)) < 2.45f) {
                            sc.term_hacked[i] = true;
                            hacked_any = true;
                            au_live_hack(&live);
                            spawn_pop(pops, v3_add(sc.term_pos[i], v3(0, 1.4f, 0)), 0, "ARCHIVE FILED", 0x68f0a8u);
                            banner_msg = (sc.term_hacked[0] && sc.term_hacked[1])
                                ? "NIGHTJAR ARCHIVE COMPLETE - EXTRACT AT WEST HARBOR BARGE!"
                                : "ARCHIVE 1/2 SECURED - PROCEED TO SECOND TERMINAL";
                            banner_t = 4.5f;
                        }
                    }
                    if (!hacked_any && ammo < 12) {
                        ammo = 12;
                        au_live_tone(&live, 0.07f, 620.0f, 920.0f, 0.09f);
                        banner_msg = "HK USP RELOADED (12 RDS)";
                        banner_t = 1.6f;
                    }
                }
            }

            /* West Harbor Barge Extraction + CRC32 Career Persistence */
            if (!extracted && sc.term_hacked[0] && sc.term_hacked[1]) {
                if (v3_len(v3_sub(player.pos, v3(-11.5f, 0.0f, 18.0f))) < 3.8f) {
                    extracted = true;
                    show_dossier = 1;
                    au_live_hack(&live);
                    banner_msg = "EXTRACTION CONFIRMED - HANDLER DEBRIEF FILED";
                    banner_t = 8.0f;
                    if (best_alert_record < 0.0f || alert_peak < best_alert_record)
                        best_alert_record = alert_peak;
                    SaveState sv = { 0, 0, player.pos, best_alert_record, (uint32_t)frames, 0 };
                    save_write("debrief_save.dat", &sv);
                }
            }

            noise = fmaxf(noise * 0.90f,
                          player.grappling ? 0.35f :
                          !moving ? 0.0f :
                          crouched ? 0.06f :
                          (in.keys & K_SPRINT) ? 0.82f : 0.28f);

            V3 prev = player.pos;
            wall_running = physics_wallrun_step(&player, &wall_norm, &wall_timer,
                                                sc.world, sc.nworld, DT);
            if (!wall_running && moving)
                physics_mantle_step(&player, wish, sc.world, sc.nworld, DT);
            physics_step(&player, sc.world, sc.nworld, DT);

            float moved = v3_len(v3_sub(player.pos, prev));
            if ((player.grounded || wall_running) && moving && slide_timer <= 0.0f) {
                step_acc += moved;
                if (step_acc > (crouched ? 2.2f : 1.80f)) {
                    step_acc = 0.0f;
                    if (!crouched) au_live_footstep(&live, !wall_running);
                }
            }

            float vis = light_exposure_at(&sc.rig, v3(player.pos.x, player.pos.y + (crouched ? 0.65f : 1.15f), player.pos.z));
            if (crouched) vis *= 0.62f;

            /* Social stealth disguise fools troopers at >4.5m when walking calmly,
             * while Patrol Mecha biometric sensor always reads true exposure! */
            for (int g = 0; g < 2; g++) {
                float g_vis = vis;
                float g_noise = noise;
                if (disguised && !(in.keys & K_SPRINT) && !aiming &&
                    v3_len(v3_sub(guards[g].pos, player.pos)) > 4.5f) {
                    g_vis *= 0.14f;
                    g_noise *= 0.25f;
                }
                guard_update_ex(&guards[g], g == 0 ? GUARD0_WP : GUARD1_WP, 4,
                                player.pos, g_noise, g_vis, sc.world, sc.nworld, DT);
            }
            mecha_update(&walker, MECHA_WP, 4, player.pos, noise, vis, sc.world, sc.nworld, DT);
            if (walker.step_event) {
                float mg, mpan;
                au_positional(v3_len(v3_sub(walker.pos, player.pos)), &mg, &mpan);
                au_live_mecha_step(&live, mg * 1.1f);
            }

            float max_al = fmaxf(fmaxf(guards[0].alert, guards[1].alert), walker.alert);
            if (max_al > alert_peak) alert_peak = max_al;
            if (max_al > 0.72f && !alarmed) {
                alarmed = true;
                alarms++;
                au_live_tone(&live, 0.35f, 660.0f, 880.0f, 0.14f);
            }
            if (max_al < 0.30f) alarmed = false;

            if (showcase_front) {
                V3 front_pos = v3(player.pos.x + sinf(char_yaw) * 1.65f,
                                  player.pos.y + 1.32f,
                                  player.pos.z + cosf(char_yaw) * 1.65f);
                cam.pos = camera_resolve(front_pos, sc.world, sc.nworld, 0.25f);
                cam.yaw = char_yaw + PS15_PI;
                cam.pitch = -0.04f;
            } else {
                cam = tps_camera_step(cam, &player, cam_yaw, cam_pitch,
                                      crouched, aiming, sc.world, sc.nworld, DT);
            }
        }

        /* Seamless Title Banner -> Over-the-Shoulder Camera Blend */
        Camera render_cam = cam;
        if (title_blend > 0.001f) {
            V3 hero_pos = v3(player.pos.x - 0.63f, player.pos.y + 1.38f, player.pos.z - 0.97f);
            float s = title_blend * title_blend * (3.0f - 2.0f * title_blend);
            render_cam.pos   = v3(lerpf(cam.pos.x, hero_pos.x, s),
                                  lerpf(cam.pos.y, hero_pos.y, s),
                                  lerpf(cam.pos.z, hero_pos.z, s));
            render_cam.yaw   = lerpf(cam.yaw, 0.22f, s);
            render_cam.pitch = lerpf(cam.pitch, 0.02f, s);
            fb->settings.fov_scale = lerpf(aiming ? 1.22f : 1.0f, 1.28f, s);
        } else {
            fb->settings.fov_scale = aiming ? 1.22f : 1.0f;
        }

        double r0 = now_s();
        float tt = (float)(r0 - start);
        greyport_update_lights(&sc, guards, 2, tt);
        if (title_blend > 0.05f && sc.rig.nlights < GREYPORT_MAX_LIGHTS) {
            int li = sc.rig.nlights++;
            sc.lights[li] = (PointLight){
                v3(1.10f, 2.20f, 11.80f),
                5.8f, 1.35f * title_blend, 1.05f * title_blend, 0.70f * title_blend
            };
        }

        greyport_render_opaque(fb, &sc, &render_cam, tt);

        /* Grounded contact shadows */
        float op_gy = world_ground_y(sc.world, sc.nworld, v3(player.pos.x, player.pos.y + 0.2f, player.pos.z));
        render_blob_shadow(fb, &render_cam, &sc.shadow_quad, &sc.tex[TEX_SHADOW],
                           v3(player.pos.x, op_gy, player.pos.z), 0.58f);
        for (int i = 0; i < 2; i++)
            render_blob_shadow(fb, &render_cam, &sc.shadow_quad, &sc.tex[TEX_SHADOW], guards[i].pos, 0.58f);
        render_blob_shadow(fb, &render_cam, &sc.shadow_quad, &sc.tex[TEX_SHADOW], walker.pos, 1.25f);

        /* Operator full-body animation state machine */
        float hspd = hypotf(player.vel.x, player.vel.z);
        Pose op;
        if (title_active || title_blend > 0.5f) {
            op = anim_sample(&CLIP_AIM, tt * 0.65f);
            op.head_yaw   = (render_cam.yaw + PS15_PI) - char_yaw;
            op.head_pitch = 0.04f;
        } else if (slide_timer > 0.0f) {
            op = anim_sample(&CLIP_CROUCH, tt);
            op.crouch = 1.0f; op.lean = -0.32f; op.leg_r = -0.95f; op.knee_r = 0.15f;
        } else if (crouched) {
            op = anim_sample(&CLIP_CROUCH, hspd > 0.2f ? tt * 1.5f : 0.0f);
        } else if (wall_running) {
            op = anim_sample(&CLIP_RUN, tt * 1.6f);
            op.lean = 0.24f;
        } else if (hspd > 4.3f) {
            op = anim_sample(&CLIP_RUN, tt * 1.35f);
        } else if (hspd > 0.25f) {
            op = anim_sample(&CLIP_WALK, tt * 1.6f);
        } else {
            op = anim_sample(&CLIP_IDLE, tt);
        }
        if (!title_active && (aiming || recoil > 0.05f || showcase_front)) {
            Pose ap = anim_sample(&CLIP_AIM, tt);
            op.arm_l = ap.arm_l - cam_pitch * 0.55f;
            op.arm_r = ap.arm_r - cam_pitch * 0.55f - recoil * 0.22f;
            op.elbow_l = ap.elbow_l;
            op.elbow_r = ap.elbow_r;
            op.aim = 1.0f;
            op.head_yaw = clampf(cam_yaw - char_yaw, -0.6f, 0.6f);
        }
        if (player.grappling) {
            op.arm_r = -2.35f; op.arm_l = -2.05f; op.aim = 0.0f;
        }
        render_humanoid(fb, &render_cam, disguised ? &soldier : &oper,
                        player.pos, char_yaw, &op,
                        sc.tex, 1.0f, 1.0f, 1.02f, &sc.rig);

        if (player.grappling) {
            V3 hand = v3(player.pos.x + sinf(char_yaw) * 0.25f,
                         player.pos.y + 1.52f,
                         player.pos.z + cosf(char_yaw) * 0.25f);
            greyport_render_grapple(fb, &sc, &render_cam, hand, player.grapple_anchor);
        }

        /* Regime troopers + autonomous Type-1 Patrol Mecha */
        for (int i = 0; i < 2; i++) {
            Pose gp = (guards[i].stunned > 0.0f) ? anim_sample(&CLIP_CROUCH, 0.0f)
                    : (guards[i].wait > 0.0f)    ? anim_sample(&CLIP_IDLE, tt)
                                                 : anim_sample(&CLIP_WALK, tt * 1.35f);
            if (guards[i].stunned > 0.0f) { gp.crouch = 1.0f; gp.lean = 0.75f; }
            else { gp.arm_r = -0.72f; gp.elbow_r = 0.65f; }
            render_humanoid(fb, &render_cam, &soldier, guards[i].pos, guards[i].yaw, &gp,
                            sc.tex, 1.0f, 1.0f, 1.0f, &sc.rig);
        }
        render_mecha(fb, &render_cam, &mmesh, &walker, sc.tex, &sc.rig);

        if (ping_timer > 0.0f) {
            float r = (1.1f - ping_timer) * 4.2f + 0.35f;
            draw_ping_ring(fb, &render_cam, ping_pos, r, 0x805818u);
        }
        if (sc.term_hacked[0] && sc.term_hacked[1] && !extracted) {
            render_glow3d(fb, &render_cam, v3(-11.5f, 1.2f, 18.0f), 1.4f, 45, 195, 115);
        }

        greyport_render_fx(fb, &sc, &render_cam, guards, 2, tt);
        if (fb->settings.visor_mode) {
            fb_apply_visor(fb, tt);
        } else if (fb->settings.profile == GFX_PS2) {
            fb_bloom(fb, 0.70f, 0.32f);
        }

        if (title_active) {
            /* Live 3D DeBrief Banner Screen (left emblem + footer + right live Operator) */
            draw_debrief_emblem(fb, 22, 38);
            draw_debrief_footer(fb);
            float pulse = 0.5f + 0.5f * sinf(tt * 3.5f);
            uint32_t prompt_c = pack3(100 + (int)(110.0f * pulse), 215 + (int)(40.0f * pulse), 160 + (int)(60.0f * pulse));
            raster_text(fb, 30, 122, "[PRESS SPACE / LMB TO DEPLOY]", prompt_c);
        } else {
            /* World-space Trooper & Mecha suspicion indicators (? / !) */
            for (int i = 0; i < 2; i++) {
                if (guards[i].stunned > 0.0f || guards[i].alert < 0.16f) continue;
                int gx, gy; float gz;
                if (render_project(fb, &render_cam, v3(guards[i].pos.x, guards[i].pos.y + 2.08f, guards[i].pos.z), &gx, &gy, &gz) && gz < 36.0f) {
                    bool hi = guards[i].alert >= 0.72f;
                    raster_text(fb, gx - 8, gy - 6, hi ? "[!]" : "[?]", hi ? 0xff4838u : 0xffbe3cu);
                }
            }
            /* World-projected Mecha SIGINT telemetry callout */
            int mx, my; float mz;
            if ((aiming || fb->settings.visor_mode || walker.alert > 0.20f) &&
                render_project(fb, &render_cam, v3(walker.pos.x, walker.pos.y + 2.2f, walker.pos.z), &mx, &my, &mz) &&
                mz < 36.0f) {
                if (aiming || fb->settings.visor_mode) {
                    mecha_scanned = true;
                    raster_rect(fb, mx - 1, my - 1, 3, 3, 0xffbe3cu);
                    raster_line2d(fb, mx + 2, my, mx + 14, my - 8, 0xd69428u);
                    raster_rect(fb, mx + 14, my - 8, 18, 1, 0xd69428u);
                    char mtag[40];
                    if (walker.coolant_hp <= 0.01f)
                        snprintf(mtag, sizeof mtag, "GOVERNOR OFFLINE");
                    else if (walker.sensor_hp < walker.actuator_hp)
                        snprintf(mtag, sizeof mtag, "SENSOR %d%%", (int)(walker.sensor_hp * 100.0f));
                    else
                        snprintf(mtag, sizeof mtag, "ACTUATOR %d%%", (int)(walker.actuator_hp * 100.0f));
                    raster_text(fb, mx + 16, my - 15, mtag, 0xffbe3cu);
                } else {
                    raster_text(fb, mx - 8, my - 18, walker.alert >= 0.72f ? "[!]" : "[?]",
                                walker.alert >= 0.72f ? 0xff4838u : 0xffbe3cu);
                }
            }

            /* Floating amber visor hit numbers & event tags */
            for (int i = 0; i < 8; i++) {
                if (pops[i].timer <= 0.0f) continue;
                V3 wpos = v3_add(pops[i].pos, v3(0.0f, (1.20f - pops[i].timer) * 0.85f, 0.0f));
                int sx, sy;
                if (render_project(fb, &render_cam, wpos, &sx, &sy, NULL)) {
                    if (pops[i].tag) raster_text(fb, sx - 12, sy - 7, pops[i].tag, pops[i].col);
                    if (pops[i].value > 0) raster_number(fb, sx - 4, sy, pops[i].value, pops[i].col);
                }
            }

            /* Over-the-shoulder reticle + dynamic hitmarker */
            if (!showcase_front) {
                int cx = FB_W / 2, cy = FB_H / 2;
                int gap = aiming ? 3 : 5;
                uint32_t rc = aiming ? 0xffbe3cu : 0x94a8c8u;
                raster_rect(fb, cx - gap - 3, cy, 3, 1, rc);
                raster_rect(fb, cx + gap + 1, cy, 3, 1, rc);
                raster_rect(fb, cx, cy - gap - 3, 1, 3, rc);
                raster_rect(fb, cx, cy + gap + 1, 1, 3, rc);
                if (hitmark_t > 0.0f) {
                    raster_line2d(fb, cx - 5, cy - 5, cx - 2, cy - 2, 0xff6238u);
                    raster_line2d(fb, cx + 5, cy - 5, cx + 2, cy - 2, 0xff6238u);
                    raster_line2d(fb, cx - 5, cy + 5, cx - 2, cy + 2, 0xff6238u);
                    raster_line2d(fb, cx + 5, cy + 5, cx + 2, cy + 2, 0xff6238u);
                }
            }

            /* Diegetic Amber Visor HUD */
            float vis_now = light_exposure_at(&sc.rig, v3(player.pos.x, player.pos.y + 1.0f, player.pos.z));
            float max_al = fmaxf(fmaxf(guards[0].alert, guards[1].alert), walker.alert);
            hud_draw_visor(fb, render_cam.yaw, op_time, ammo, vis_now, max_al, noise, player.grappling);

            /* Top-left live objective + disguise status */
            int n_hacked = (sc.term_hacked[0] ? 1 : 0) + (sc.term_hacked[1] ? 1 : 0);
            char obj_line[64];
            if (extracted)
                snprintf(obj_line, sizeof obj_line, "EXTRACTED [R: NEW OP]");
            else if (n_hacked < 2)
                snprintf(obj_line, sizeof obj_line, "ARCHIVE: %d/2 [R:HACK]", n_hacked);
            else
                snprintf(obj_line, sizeof obj_line, "EXTRACT: WEST BARGE!");
            raster_text(fb, 10, 8, obj_line, n_hacked == 2 ? 0x68f0a8u : 0xffbe3cu);
            if (disguised)
                raster_text(fb, 10, 18, "COVER: GREATCOAT", 0xffd468u);
            if (banner_t > 0.0f)
                raster_text(fb, FB_W / 2 - (int)strlen(banner_msg) * 3, 33, banner_msg, 0xffd468u);

            /* Handler Debrief Room / Dossier Screen (M / 8 or on Extraction) */
            if (show_dossier) {
                raster_rect(fb, 68, 38, 344, 176, 0x090d16u);
                raster_rect(fb, 68, 38, 344, 1, 0xffbe3cu);
                raster_rect(fb, 68, 213, 344, 1, 0xffbe3cu);
                raster_rect(fb, 68, 38, 1, 176, 0xd69428u);
                raster_rect(fb, 411, 38, 1, 176, 0xd69428u);
                raster_text(fb, 82, 48, "NSA ARCHIVE // GREYPORT HANDLER DEBRIEF", 0xffbe3cu);

                char dline[72];
                snprintf(dline, sizeof dline, "STATUS       : %s", extracted ? "EXTRACTED (WEST BARGE)" : "ACTIVE IN FIELD");
                raster_text(fb, 82, 66, dline, 0xd0d8e8u);
                snprintf(dline, sizeof dline, "PRIMARY      : %d / 2 NIGHTJAR TERMINALS", n_hacked);
                raster_text(fb, 82, 78, dline, 0xd0d8e8u);
                snprintf(dline, sizeof dline, "SECONDARY    : OVERWATCH %s | MECHA %s",
                         sc.lamp_out[0] ? "BLINDED" : "LIVE",
                         walker.coolant_hp <= 0.01f ? "DISABLED" : (mecha_scanned ? "LOGGED" : "UNSCANNED"));
                raster_text(fb, 82, 90, dline, 0xd0d8e8u);
                snprintf(dline, sizeof dline, "ALARMS       : %d    PEAK ALERT : %d%%",
                         alarms, (int)(alert_peak * 100.0f));
                raster_text(fb, 82, 102, dline, 0xd0d8e8u);
                snprintf(dline, sizeof dline, "TAKEDOWNS    : %d    SHOTS FIRED: %d", takedowns, shots_fired);
                raster_text(fb, 82, 114, dline, 0xd0d8e8u);
                if (best_alert_record >= 0.0f) {
                    snprintf(dline, sizeof dline, "CAREER BEST  : %d%% PEAK ALERT (CRC32 VERIFIED)",
                             (int)(best_alert_record * 100.0f));
                    raster_text(fb, 82, 126, dline, 0x9cb0d0u);
                }
                snprintf(dline, sizeof dline, "EVALUATION   : %s",
                         compute_grade(alarms, takedowns, shots_fired, walker.coolant_hp <= 0.01f));
                raster_text(fb, 82, 144, dline, 0x68f0a8u);
                raster_text(fb, 82, 162,
                            handler_remark(alarms, takedowns, walker.coolant_hp <= 0.01f, sc.lamp_out[0]),
                            0xffd468u);
                raster_text(fb, 82, 192, extracted ? "[R] DEPLOY NEXT OP     [M] CLOSE DOSSIER"
                                                   : "[M] RETURN TO FIELD    [N] SIGINT VISOR", 0x8ea4c8u);
            }
        }

        if (show_tech) {
            static const char *DAC_NAMES[3]  = { "555+BAYER", "555-FLAT", "RGB888" };
            static const char *GFX_NAMES[3]  = { "PS1", "PS1.5", "PS2" };
            char line[112];
            snprintf(line, sizeof line, "TRIS:%d MS:%.1f GFX:%s FILTER:%s DAC:%s",
                     fb->stats.tris_drawn, last_render_ms,
                     GFX_NAMES[fb->settings.profile],
                     fb->settings.filter_mode == FILTER_BILINEAR ? "BILIN" : "POINT",
                     DAC_NAMES[fb->settings.dither_mode]);
            raster_text(fb, 76, FB_H - 9, line, 0x7c92b4u);
        }

        last_render_ms = (float)((now_s() - r0) * 1000.0);
        present(plat, fb);
        frames++;
        if (max_frames && frames >= max_frames) break;
    }

    if (dump_path) {
        if (dump_ppm(dump_path, fb) == 0)
            printf("dumped %s (960x540)\n", dump_path);
    }

    double elapsed = now_s() - start;
    printf("session: %.1f s, %ld frames (%.0f fps), grapples=%d, alert peak %.2f, grade=%s\n",
           elapsed, frames, frames / (elapsed > 0.001 ? elapsed : 1.0),
           grapples, alert_peak,
           compute_grade(alarms, takedowns, shots_fired, walker.coolant_hp <= 0.01f));
    printf("assets: brick.ppm=%s crate.obj=%s\n",
           sc.have_brick_file ? "loaded" : "procedural fallback",
           sc.have_crate ? "loaded" : "procedural fallback");

    audev_stop();
    plat_close(plat);
    fb_free(fb);
    mecha_mesh_free(&mmesh);
    humanoid_free(&oper);
    humanoid_free(&soldier);
    greyport_free(&sc);
    return 0;
}
