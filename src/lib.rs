pub mod filmic;

#[cfg(test)]
mod tests {
    use crate::filmic::*;

    #[test]
    fn filmic() {
        0f32.filmic();
        [0f32, 0f32, 0f32].filmic();
        #[cfg(feature = "glam-support")]
        glam::Vec3::zero().filmic();
        #[cfg(feature = "spirv-std-support")]
        spirv_std::Vec3::zero().filmic();
    }
}
