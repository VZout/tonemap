<h3 align="center">
🎨 tonemap
<br><br>
  <a href="https://crates.io/crates/tonemap"><img alt="Crates.io" src="https://img.shields.io/crates/v/tonemap?style=flat-square"></a>
  <a href="https://crates.io/crates/tonemap"><img alt="Crates.io" src="https://img.shields.io/crates/l/tonemap?style=flat-square"></a>
  <a href="https://crates.io/crates/tonemap"><img alt="Crates.io" src="https://img.shields.io/crates/d/tonemap?style=flat-square"></a>
</h3>


---

<p align="center">
  <strong>
  A collection of tone mapping algorithms for Rust
  </strong>
</p>

---

## Usage

```rust
use tonemap::filmic::*;

0f32.filmic();
[0f32, 0f32, 0f32].filmic();
glam::Vec3::zero().filmic();
spirv_std::Vec3::zero().filmic();
// ...
```

## Algorithms

* [ ] Exposure Correct
* [ ] Gamma Correct
* [ ] Linear
* [ ] Reinhard
* [ ] ACES
  * High Quality
  * High Performance
* [ ] Unreal 3
* [ ] Lottes
* [ ] Uchimura
* [x] Filmic
* [ ] RomBinDaHouse
* [ ] Reinhard
  * Simple
  * Luma Based
  * White Preserving
* [ ] Haarm-Peter Duiker
* [ ] Uncharted 2 
* [ ] Sigmoid Contrast

## Third party libraries

The tone mapping traits are implemented for the following libraries as well:

| feature | library |
| - | - |
| `glam_support` | [glam](https://docs.rs/glam/0.9.5/glam/index.html) |
| `spirv-std-support` | [spirv-std](https://github.com/EmbarkStudios/rust-gpu) |
