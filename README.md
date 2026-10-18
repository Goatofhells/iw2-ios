# IW4 Pocket

An iOS 15+ SwiftUI launcher with ARM64 Rust services and a separately packaged Macroquad practice range. This is not a port of the IW4 engine. No MW2 gameplay, compatible server connection, or bots are implemented.

## Rust range

Pocket Range uses upstream Macroquad 0.4.16 with Metal and a retained Miniquad iOS backend patch. Download PocketRange-unsigned, sign it in KSign, and install it alongside the launcher. The launcher's Offline Practice page opens it using its URL scheme. See [engine/README.md](engine/README.md) for controls, features, licensing, and verification limits. Native ARM64 code needs no JIT entitlement. Compilation and IPA checks establish a device-test build; physical runtime verification is performed by the user.

## Current milestone

The app provides Home, Import Game Files, Verify Installation, a link to the companion Offline Practice app, Multiplayer status, persistent Settings, and Diagnostics. The dark launcher supports landscape iPhone and iPad layouts. It imports ZIP archives or individual FF, IWD, and CFG files into Application Support. Each selection creates a separate import set; folder import and merging sets are not supported yet.

ZIP imports reject traversal, absolute paths, symbolic links, duplicate names including case collisions, excessive entry counts, and expanded sizes above 8 GiB. Extraction uses a fresh destination and removes partial output on failure. Imports run away from the UI thread. ZIPs must be unencrypted and use stored or deflate compression. Available device storage can impose a lower practical limit.

The version 1 trusted manifest is intentionally empty because no trustworthy MW2 hashes were supplied. Verification therefore returns unsupported. The Rust verifier supports streaming SHA-256, path checks, and corruption detection, exercised using original fixtures. Hash verification does not establish Steam ownership. MW2 gameplay stays disabled independently of verification results.

Launcher Settings save future frame rate, resolution, cache cap, sensitivity, and inverted aiming preferences. These do not affect the separate range, which has its own sensitivity and invert controls. Diagnostics show device class, OS, physical RAM, imported set count, and a Rust-derived resource budget. This is not live resident-memory telemetry or a device benchmark.

## GitHub-only builds and tests

`.github/workflows/ios-release.yml` runs on pushes and manual dispatch. It uses macos-15, stable Rust, aarch64-apple-ios, Rust tests, packaging tests, XcodeGen, and xcodebuild. It links the Rust static library using the C ABI in ios/bridge.h. It packages the real application bundle and checks the IPA's plist, ARM64 Mach-O header, executable presence, deployment target, and safe paths. Structural checks do not replace installation or runtime tests.

Download IW4Pocket-unsigned and build-evidence from a successful Actions run. Build evidence contains test/build logs and Cargo.lock. No local compilation is required. Manual dispatch accepts an optional new tag and publishes a GitHub prerelease only after successful verification. No supplied baseline workflow was present in the attachment; this workflow implements its requested requirements directly.

An unsigned IPA cannot be installed directly on a physical iPhone. Installation requires appropriate Apple signing and provisioning. This repository does not manage signing credentials. A Windows USB connection does not provide an iOS compiler or prove that the app runs on that device.

## Tests and limitations

Rust tests cover known SHA-256 fixtures, corruption, malformed and unsupported manifests, safe extraction, traversal, links, case collisions, malformed ZIPs, settings bounds, memory budgets, and FFI responses. Python tests exercise IPA acceptance/rejection and build configuration. GitHub Actions is the source of test and compilation results; source files alone do not constitute a passing build.

No app icon, device benchmark, signing, release signing automation, controller controls, touch layout editor, or import deletion UI is included. The service interfaces are experimental. Cargo.lock retains exact resolved versions and Rust builds and tests use --locked. Simulator UI testing is an optional manual-dispatch input and is disabled for normal pushes at the user's request. The first simulator launch failed due to a missing Metal resize callback; the backend patch addresses the crash report, but its runtime fix is not claimed as verified until a device test succeeds.

## Licensing and next milestones

No proprietary engine code, executable, assets, maps, sounds, or textures are bundled. Users must have lawful access to any files they import. Original MW2 files remain subject to their owners' rights; ownership of files does not grant redistribution or engine-porting rights. No DRM, authentication, anti-cheat, or server restrictions are bypassed.

The launcher foundation passed GitHub builds and the user confirmed it runs on their iPhone. The next slice is the Macroquad original range, followed by physical-device range testing, a tested bot, experimental authoritative networking, and MW2 compatibility research. No lawful compatible IW4 runtime is assumed to exist.
