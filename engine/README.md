# Rust practice runtime

`range` uses upstream Macroquad, pinned to 0.4.16 from crates.io. Its Miniquad backend is explicitly configured for Apple's Metal API.

- Upstream: https://github.com/not-fl3/macroquad
- Official iOS instructions: https://macroquad.rs/articles/ios/
- License: MIT OR Apache-2.0. Dependency license texts are collected from resolved source packages and bundled as ThirdPartyNotices.txt in the IPA.

Macroquad owns the native application lifecycle. Pocket Range is a separate application (`com.iw4pocket.range`, URL scheme `pocketrange`) opened from the SwiftUI launcher. Import data stays in the launcher's sandbox. The companion uses no imported MW2 content.

The range has an original 3D arena, multi-touch movement and aiming, solid obstacles, ray-tested targets, semi-automatic fire, 12-round magazines, unlimited timed reloads, jumping, crouching, sprint, aim zoom, pause/resume, sensitivity and invert controls, and 60-second rounds with hits, shots, and accuracy. Targets reappear after 1.8 seconds. These are stationary targets, not AI bots. Range settings apply immediately but are not persisted. Launcher preferences do not control the companion app.

Controls: left-side floating joystick; drag unused right-side space to aim; FIRE, LOAD, JUMP, LOW, RUN, AIM, and PAUSE buttons. Desktop diagnostic controls are WASD, arrows, F, R, Space, C, Shift, Control, and Escape.

GitHub runs simulation tests, builds device and simulator binaries, installs the simulator app, and runs XCTest taps. The simulator must record a started round, a real shot, and pause in its diagnostic file. XCTest screenshots are retained in the xcresult artifact. A simulator run does not establish physical iPhone performance or signing compatibility.

Not implemented: IW4/MW2 compatibility, imported maps, bots, multiplayer, sound, controllers, textured art, touch-layout editing, persistent range preferences, or a benchmark. The geometric range is the actual playable environment.
