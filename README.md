# IW4 Pocket

An iOS 15+ SwiftUI launcher foundation with ARM64 Rust services. This is not a port of the IW4 engine. No MW2 gameplay, compatible server connection, renderer, bots, or original FPS prototype is implemented.

## Current milestone

The app provides Home, Import Game Files, Verify Installation, Offline Practice status, Multiplayer status, persistent Settings, and Diagnostics. The dark launcher supports landscape iPhone and iPad layouts. It imports ZIP archives or individual FF, IWD, and CFG files into Application Support. Each selection creates a separate import set; folder import and merging sets are not supported yet.

ZIP imports reject traversal, absolute paths, symbolic links, duplicate names including case collisions, excessive entry counts, and expanded sizes above 8 GiB. Extraction uses a fresh destination and removes partial output on failure. Imports run away from the UI thread. ZIPs must be unencrypted and use stored or deflate compression. Available device storage can impose a lower practical limit.

The version 1 trusted manifest is intentionally empty because no trustworthy MW2 hashes were supplied. Verification therefore returns unsupported. The Rust verifier supports streaming SHA-256, path checks, and corruption detection, exercised using original fixtures. Hash verification does not establish Steam ownership. MW2 gameplay stays disabled independently of verification results.

Settings save future frame rate, resolution, cache cap, sensitivity, and inverted aiming preferences. These have no gameplay effect until a runtime exists. Diagnostics show device class, OS, physical RAM, imported set count, and a Rust-derived resource budget. This is not live resident-memory telemetry or a device benchmark.

## GitHub-only builds and tests

`.github/workflows/ios-release.yml` runs on pushes and manual dispatch. It uses macos-15, stable Rust, aarch64-apple-ios, Rust tests, packaging tests, XcodeGen, and xcodebuild. It links the Rust static library using the C ABI in ios/bridge.h. It packages the real application bundle and checks the IPA's plist, ARM64 Mach-O header, executable presence, deployment target, and safe paths. Structural checks do not replace installation or runtime tests.

Download IW4Pocket-unsigned and build-evidence from a successful Actions run. Build evidence contains test/build logs and Cargo.lock. No local compilation is required. Manual dispatch accepts an optional new tag and publishes a GitHub prerelease only after successful verification. No supplied baseline workflow was present in the attachment; this workflow implements its requested requirements directly.

An unsigned IPA cannot be installed directly on a physical iPhone. Installation requires appropriate Apple signing and provisioning. This repository does not manage signing credentials. A Windows USB connection does not provide an iOS compiler or prove that the app runs on that device.

## Tests and limitations

Rust tests cover known SHA-256 fixtures, corruption, malformed and unsupported manifests, safe extraction, traversal, links, case collisions, malformed ZIPs, settings bounds, memory budgets, and FFI responses. Python tests exercise IPA acceptance/rejection and build configuration. GitHub Actions is the source of test and compilation results; source files alone do not constitute a passing build.

No app icon, device benchmark, simulator UI tests, signing, release signing automation, controller controls, touch layout editor, or import deletion UI is included in this milestone. The initial service interfaces are experimental. Dependency versions use Cargo-compatible ranges; retain the generated Cargo.lock from the first successful build for repeatability in the next revision.

## Licensing and next milestones

No proprietary engine code, executable, assets, maps, sounds, or textures are bundled. Users must have lawful access to any files they import. Original MW2 files remain subject to their owners' rights; ownership of files does not grant redistribution or engine-porting rights. No DRM, authentication, anti-cheat, or server restrictions are bypassed.

Complete milestones 1–5 with a passing GitHub build before continuing. Milestone 6 is IPA verification and signed device testing. Then implement a minimal original 3D renderer and touch movement, an original arena, one tested bot, experimental authoritative networking, and documented MW2 compatibility research. Compatibility research has not been completed and no lawful compatible runtime is assumed to exist.
