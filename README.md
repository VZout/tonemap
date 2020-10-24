<h3 align="center">
🎨 tonemap
<br><br>
  <img alt="Crates.io" src="https://img.shields.io/crates/v/tonemap?style=flat-square">
  <img alt="Crates.io" src="https://img.shields.io/crates/l/tonemap?style=flat-square">
  <img alt="Crates.io" src="https://img.shields.io/crates/d/tonemap?style=flat-square">
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
0f32.filmic();
[0f32, 0f32, 0f32].filmic();
glam::Vec3::zero().filmic();
// ...
```

## Algorithms

* [x] Filmic
* [ ] Reinhard
* [ ] ACES

## Third party libraries

The tone mapping traits are implemented for the following libraries as well:

| feature | library |
| - | - |
| `glam_support` | [glam](https://docs.rs/glam/0.9.5/glam/index.html) |
