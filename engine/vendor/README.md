# Miniquad iOS input patch

`miniquad` is the upstream 0.4.11 crate archive from https://crates.io/api/v1/crates/miniquad/0.4.11/download.
Original archive SHA-256: dcbaea978d741f3b60ced666b06a2a1e4a2bc81cb8b9dd29f4a40965e1f59e65.
Upstream license files and source attribution are preserved.

Only src/native/ios.rs is patched: use each UITouch's actual phase rather than assigning a callback-wide phase to allTouches; skip stationary touches; enable multiple touches on the Metal view; honor the configured high-DPI setting on that view. Cargo's patch table makes this same source available to every build without modifying a global registry cache. The game still uses the real Macroquad engine.

The patch also implements the required MTKViewDelegate `mtkView:drawableSizeWillChange:` callback. The simulator's crash report identified a missing selector during MTKView resize. The callback updates display dimensions and notifies the event loop after initialization, while allowing UIKit's early initialization resize. Runtime verification of this fix is left to the user's iPhone test; the simulator test is optional.

The Metal view requests the same Depth32Float_Stencil8 depth/stencil format as Miniquad's render pipelines and its existing macOS backend, allowing the 3D range to use depth testing.
