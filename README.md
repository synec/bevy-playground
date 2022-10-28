# bevy-playground

My playground for [Bevy](https://bevyengine.org).

Setup follows the recommendations of [Bevy book](https://bevyengine.org/learn/book/getting-started/setup/) regarding performance optimizations, but does not use dynamic linking.

The demo Scene is based on the [3D scene](<(https://bevyengine.org/examples/3d/3d-scene/)>) from [Bevy examples](https://bevyengine.org/examples) and adds/changes the following

- `main.rs` allows unused definitions
- Added `LogDiagnosticsPlugin`
- Added `FrameTimeDiagnosticsPlugin`
- System to rotate the cube

The repository is meant to be a template for future Bevy projects.
