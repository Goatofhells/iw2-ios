# Foundation build report

Implemented the initial repository, SwiftUI launcher, Rust services, automated tests, and GitHub release workflow. The first complete GitHub run succeeded on October 18, 2026:

https://github.com/Goatofhells/iw2-ios/actions/runs/39135133169

- Four Rust integration tests passed, covering hashing, manifests, corrupted files, archive security, configuration, memory limits, and the C ABI through multiple assertions.
- Two Python tests passed, covering build settings and IPA structural checks with five synthetic acceptance/rejection cases.
- Zero test failures.
- ARM64 Rust static library built successfully.
- Xcode compiled the SwiftUI application for iOS 15.0+ successfully.
- Unsigned IPA packaged, structurally verified, and uploaded with build evidence.
- No local builds or local test execution were performed.

The follow-up revision retains the generated Cargo.lock, enforces locked Rust dependencies, uses lowercase custom type names per workspace rules, and explicitly requests full-screen landscape presentation. Its GitHub run must also pass before treating that revision as verified.

Created source and build files: Cargo.toml, Cargo.lock, ios/pocket.swift, ios/bridge.h, ios/project.yml; launcher, validation, and diag crates; Rust integration tests; tools/test_build.py and tools/verify_ipa.py; game/manifest.json; .github/workflows/ios-release.yml; repository, engine, game, and server documentation; and .gitignore. Xcode project and Info.plist are generated on GitHub and saved in build evidence.

Not implemented: an IW4 engine port, 3D rendering, playable prototype, bots, touch gameplay, controllers, multiplayer, MW2 server compatibility, or complete compatibility research. The manifest is intentionally empty, so no MW2 installation is reported as verified. Imported files never enable gameplay. No copyrighted game content is bundled or downloaded, and no ownership or redistribution rights are inferred from file hashes.

The connected iPhone has not run this app. Unsigned IPA installation requires appropriate Apple code signing and provisioning. Structural verification is not a device runtime test. Next milestone: sign and validate the launcher on an iPhone, then build and test an original rendering and touch-movement slice.
