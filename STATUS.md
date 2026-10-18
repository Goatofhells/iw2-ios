# Rust range device-test build

The user confirmed the original launcher works on their iPhone. Macroquad 0.4.16 now provides the original Rust target range through a separate native app. The launcher opens it using the pocketrange URL scheme. Miniquad 0.4.11 is retained with its upstream licenses and focused iOS input, resize, and depth-buffer patches.

Verified GitHub build: https://github.com/Goatofhells/iw2-ios/actions/runs/39139755816
Source revision: b4cb1263b74ed4bb6e512a7d4c6f09215b5dcaae

Eight Rust tests and two Python packaging tests passed. GitHub built the ARM64 Rust services, SwiftUI launcher, and Rust range. Both unsigned IPAs passed structural verification. No local project builds or tests were run. This proves compilation and packaging, not gameplay on a physical device.

The earlier simulator attempt failed to keep the app open. Its crash report identified a missing required Metal resize selector, now implemented. The iOS Metal view also enables multiple touches, handles each touch's actual phase, honors render scale, and requests the depth/stencil format expected by the renderer. Runtime verification of these fixes remains pending the user's iPhone test. At the user's request, simulator UI tests are now optional and disabled on normal pushes. No simulator success is claimed.

The range implements touch movement and aiming, obstacle collision, target ray hits and respawn, shooting, 12-round magazines, unlimited timed reloads, jumping, crouching, sprint, aim zoom, pause, and 60-second rounds with hit and shot counts. Range sensitivity and invert controls apply immediately but are not persisted. Launcher preferences do not control the companion range. No JIT entitlement is required by this native ARM64 build.

Both version 0.2.0 IPAs and START HERE.txt were copied over USB into KSign Documents/IW4 Pocket/0.2.0. Both IPAs were pulled back and their SHA-256 hashes matched the originals:

- PocketRange-unsigned.ipa: 494190 bytes; 24a45aae90e8528bb8e2799d900c7e03ba77f3568c91a25ddf8f8eb1dc406b80
- IW4Pocket-unsigned.ipa: 406100 bytes; 7133be4528bde9f8929c8b68049d838bf8774067b7b19408d89be131a347d95d

The user signs and installs with KSign. Neither the new range nor the updated launcher is claimed installed or tested on the physical phone yet. No IW4/MW2 runtime, imported-map support, bots, multiplayer, sound, controller support, or performance benchmark is included.
