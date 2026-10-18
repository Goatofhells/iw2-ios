use macroquad::prelude::*;
use macroquad::miniquad;
use range::{input, marks, state, walls};

fn main() {
    macroquad::Window::from_config(Conf {
        window_title: "Pocket Range".into(),
        window_width: 1200,
        window_height: 700,
        high_dpi: false,
        fullscreen: true,
        platform: miniquad::conf::Platform {
            apple_gfx_api: miniquad::conf::AppleGfxApi::Metal,
            ..Default::default()
        },
        ..Default::default()
    }, async {
        simulate_mouse_with_touch(false);
        let mut game = state::default();
        let mut mode = 0;
        let mut prev = std::collections::HashMap::<u64, Vec2>::new();
        let mut roles = std::collections::HashMap::<u64, usize>::new();
        let mut base = Vec2::ZERO;
        let mut stick = Vec2::ZERO;
        let mut sense = 1.;
        let mut invert = false;
        let mut last = get_time();
        let mut report = (99, u32::MAX, u32::MAX);
        loop {
            let now = get_time();
            let gap = now - last;
            let dt = get_frame_time().min(0.05);
            last = now;
            let w = screen_width();
            let h = screen_height();
            let unit = (h / 430.).min(w / 850.);
            let pad = 40. * unit;
            let rad = 31. * unit;
            let btns = [
                (vec2(w - pad - 45. * unit, h - 80. * unit), "FIRE"),
                (vec2(w - pad - 125. * unit, h - 45. * unit), "LOAD"),
                (vec2(w - pad - 40. * unit, h - 165. * unit), "JUMP"),
                (vec2(w - pad - 135. * unit, h - 130. * unit), "LOW"),
                (vec2(pad + 155. * unit, h - 55. * unit), "RUN"),
                (vec2(w - pad - 210. * unit, h - 70. * unit), "AIM"),
                (vec2(w - pad, 38. * unit), "PAUSE"),
            ];
            let mut cmd = input::default();
            let mut press = None;
            let taps = touches();
            if gap > 0.5 && mode == 1 { mode = 2; roles.clear(); prev.clear(); }
            for tap in &taps {
                if tap.phase == TouchPhase::Started {
                    if mode == 1 {
                        let pick = btns.iter().position(|(pos, _)| tap.position.distance(*pos) < rad * 1.25);
                        let role = pick.unwrap_or(if tap.position.x < w * 0.4 { 7 } else { 8 });
                        if role == 7 {
                            if roles.values().any(|val| *val == 7) { continue; }
                            base = tap.position;
                        }
                        roles.insert(tap.id, role);
                        prev.insert(tap.id, tap.position);
                        match role { 0 => cmd.fire = true, 1 => cmd.reload = true, 2 => cmd.jump = true,
                            3 => cmd.crouch = true, 6 => { mode = 2; }, _ => {} }
                    } else { press = Some(tap.position); }
                }
                if matches!(tap.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                    roles.remove(&tap.id);
                    prev.remove(&tap.id);
                    continue;
                }
                if let Some(role) = roles.get(&tap.id) {
                    match *role {
                        4 => cmd.sprint = true,
                        5 => cmd.aim = true,
                        7 => {
                            stick = (tap.position - base).clamp_length_max(55. * unit);
                            cmd.walk = vec2(stick.x, -stick.y) / (55. * unit);
                        }
                        8 => {
                            let old = prev.insert(tap.id, tap.position).unwrap_or(tap.position);
                            let delta = (tap.position - old) / w * sense * if cmd.aim { 1.5 } else { 3. };
                            cmd.look += vec2(delta.x, delta.y * if invert { 1. } else { -1. });
                        }
                        _ => {}
                    }
                }
            }
            if !roles.values().any(|val| *val == 7) { stick = Vec2::ZERO; }
            if is_mouse_button_pressed(MouseButton::Left) && mode != 1 { press = Some(mouse_position().into()); }
            if mode == 1 {
                cmd.walk += vec2((is_key_down(KeyCode::D) as i32 - is_key_down(KeyCode::A) as i32) as f32,
                    (is_key_down(KeyCode::W) as i32 - is_key_down(KeyCode::S) as i32) as f32);
                cmd.look += vec2((is_key_down(KeyCode::Right) as i32 - is_key_down(KeyCode::Left) as i32) as f32 * dt * 1.8,
                    (is_key_down(KeyCode::Up) as i32 - is_key_down(KeyCode::Down) as i32) as f32 * dt * 1.8);
                cmd.fire |= is_key_pressed(KeyCode::F);
                cmd.reload |= is_key_pressed(KeyCode::R);
                cmd.jump |= is_key_pressed(KeyCode::Space);
                cmd.crouch |= is_key_pressed(KeyCode::C);
                cmd.sprint |= is_key_down(KeyCode::LeftShift);
                cmd.aim |= is_key_down(KeyCode::LeftControl);
                if is_key_pressed(KeyCode::Escape) { mode = 2; }
            }
            let aim = cmd.aim;
            if mode == 1 { game.step(dt, cmd); }
            if game.time <= 0. && mode == 1 { mode = 3; }
            clear_background(Color::from_hex(0x182630));
            let eye = game.pos + vec3(0., if game.low { 0.9 } else { 1.65 }, 0.);
            let dir = vec3(game.yaw.sin() * game.pitch.cos(), game.pitch.sin(), -game.yaw.cos() * game.pitch.cos());
            let right = vec3(game.yaw.cos(), 0., game.yaw.sin());
            set_camera(&Camera3D { position: eye, target: eye + dir, up: Vec3::Y,
                fovy: if aim { 42_f32.to_radians() } else { 70_f32.to_radians() }, z_near: 0.03, z_far: 100., ..Default::default() });
            draw_plane(Vec3::ZERO, vec2(20., 24.), None, Color::from_hex(0x33434a));
            for idx in -10..=10 {
                draw_line_3d(vec3(idx as f32, 0.005, -12.), vec3(idx as f32, 0.005, 12.), Color::from_hex(0x42545a));
            }
            for idx in -12..=12 {
                draw_line_3d(vec3(-10., 0.005, idx as f32), vec3(10., 0.005, idx as f32), Color::from_hex(0x42545a));
            }
            for (idx, (mid, size)) in walls.iter().enumerate() {
                draw_cube(*mid, *size, None, if idx < 4 { Color::from_hex(0x53636b) } else { Color::from_hex(0x8b704b) });
                draw_cube_wires(*mid, *size, Color::from_hex(0x192a32));
            }
            for (idx, mid) in marks.iter().enumerate() {
                draw_cube(vec3(mid.x, 0.5, mid.z), vec3(0.12, 1., 0.12), None, DARKGRAY);
                if game.dead[idx] <= 0. {
                    draw_cube(*mid, vec3(0.8, 1.2, 0.2), None, Color::from_hex(0xe1a74b));
                    draw_cube(*mid + vec3(0., 0., 0.12), vec3(0.35, 0.45, 0.03), None, Color::from_hex(0xa83d35));
                    draw_cube_wires(*mid, vec3(0.8, 1.2, 0.2), Color::from_hex(0x291d19));
                }
            }
            if mode == 1 {
                let gun = eye + dir * 0.45 + right * if aim { 0. } else { 0.16 } - Vec3::Y * (0.15 + game.reload.sin().abs() * 0.12);
                draw_cube(gun, vec3(0.09, 0.09, 0.28), None, Color::from_hex(0x202b31));
                if game.flash > 0. {
                    draw_sphere(gun + dir * 0.2, 0.045, None, YELLOW);
                    draw_line_3d(gun + dir * 0.2, game.beam, Color::new(1., 0.8, 0.3, 0.6));
                }
            }
            set_default_camera();
            draw_rectangle(0., 0., w, 65. * unit, Color::new(0.03, 0.07, 0.1, 0.9));
            draw_text("POCKET / RANGE", pad, 27. * unit, 25. * unit, WHITE);
            draw_text(&format!("{:02.0}s   HITS {}   SHOTS {}   {} / 12", game.time.ceil(), game.score, game.shots, game.ammo), pad, 51. * unit, 20. * unit, Color::from_hex(0xe1a74b));
            if mode == 1 {
                let col = if game.hit > 0. { RED } else { WHITE };
                draw_line(w * 0.5 - 8., h * 0.5, w * 0.5 + 8., h * 0.5, 2., col);
                draw_line(w * 0.5, h * 0.5 - 8., w * 0.5, h * 0.5 + 8., 2., col);
                for (idx, (pos, text)) in btns.iter().enumerate() {
                    draw_circle(pos.x, pos.y, rad, Color::new(0.04, 0.1, 0.13, 0.8));
                    draw_circle_lines(pos.x, pos.y, rad, 1.5, if idx == 0 { ORANGE } else { GRAY });
                    let dim = measure_text(text, None, (14. * unit) as u16, 1.);
                    draw_text(text, pos.x - dim.width * 0.5, pos.y + 5. * unit, 14. * unit, WHITE);
                }
                let pos = if roles.values().any(|val| *val == 7) { base } else { vec2(pad + 60. * unit, h - 90. * unit) };
                draw_circle_lines(pos.x, pos.y, 55. * unit, 2., GRAY);
                draw_circle(pos.x + stick.x, pos.y + stick.y, 22. * unit, Color::new(0.8, 0.86, 0.9, 0.5));
                if game.reload > 0. { draw_text("RELOADING", w * 0.5 - 65. * unit, h * 0.6, 22. * unit, ORANGE); }
            } else {
                roles.clear(); prev.clear();
                draw_rectangle(0., 65. * unit, w, h, Color::new(0.02, 0.05, 0.08, 0.88));
                let x = w * 0.25;
                draw_text(if mode == 3 { "ROUND COMPLETE" } else if mode == 2 { "PAUSED" } else { "RUST. METAL. LIVE." }, x, h * 0.27, 35. * unit, WHITE);
                draw_text("Macroquad 0.4.16 / original target range", x, h * 0.35, 20. * unit, ORANGE);
                draw_text("Left thumb: move. Right drag: aim. Tap FIRE to shoot.", x, h * 0.42, 18. * unit, LIGHTGRAY);
                draw_text("12 rounds / unlimited reloads / 60 seconds", x, h * 0.48, 18. * unit, LIGHTGRAY);
                if mode == 3 {
                    let rate = if game.shots > 0 { game.score * 100 / game.shots } else { 0 };
                    draw_text(&format!("{} hits / {} shots / {}% accuracy", game.score, game.shots, rate), x, h * 0.54, 22. * unit, WHITE);
                }
                let items = [
                    (Rect::new(x, h * 0.59, w * 0.5, 42. * unit), if mode == 2 { "RESUME" } else { "START RANGE" }.to_string()),
                    (Rect::new(x, h * 0.72, w * 0.24, 35. * unit), format!("SENSITIVITY {:.1}", sense)),
                    (Rect::new(w * 0.51, h * 0.72, w * 0.24, 35. * unit), format!("INVERT {}", if invert { "ON" } else { "OFF" })),
                ];
                for (idx, (rect, text)) in items.iter().enumerate() {
                    draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::from_hex(if idx == 0 { 0xa16c28 } else { 0x253b47 }));
                    draw_text(text, rect.x + 12. * unit, rect.y + rect.h * 0.67, 20. * unit, WHITE);
                    if press.is_some_and(|pos| rect.contains(pos)) {
                        match idx {
                            0 => { if mode != 2 { game = state::default(); } mode = 1; },
                            1 => { sense += 0.5; if sense > 2. { sense = 0.5; } },
                            2 => invert = !invert,
                            _ => {}
                        }
                    }
                }
                draw_text("No MW2 assets. No bots or networking in this build.", x, h * 0.88, 16. * unit, GRAY);
            }
            let snap = (mode, game.shots, game.score);
            if snap != report {
                if let Ok(root) = std::env::var("HOME") {
                    let path = std::path::Path::new(&root).join("Documents");
                    if path.is_dir() {
                        if let Err(err) = std::fs::write(path.join("range-status.json"),
                            format!("{{\"mode\":{},\"shots\":{},\"hits\":{},\"engine\":\"macroquad 0.4.16\",\"backend\":\"Metal\"}}", mode, game.shots, game.score)) {
                            eprintln!("Range diagnostics: {err}");
                        }
                    }
                }
                report = snap;
            }
            next_frame().await;
        }
    });
}
