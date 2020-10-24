pub trait Filmic {
    fn filmic(self) -> Self;
}

impl Filmic for f32 {
    fn filmic(self) -> f32 {
        let v = 0f32.max(self - 0.004);
        (v * (6.2 * v + 0.5)) / (v * (6.2 * v + 1.7) + 0.06)
    }
}

impl Filmic for [f32; 3] {
    fn filmic(self) -> [f32; 3] {
        [self[0].filmic(), self[1].filmic(), self[2].filmic()]
    }
}

#[cfg(feature = "glam_support")]
impl Filmic for glam::Vec3 {
    fn filmic(self) -> glam::Vec3 {
        glam::Vec3::new(self.x().filmic(), self.y().filmic(), self.z().filmic())
    }
}
