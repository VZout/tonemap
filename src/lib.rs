pub mod filmic;
pub use filmic::*;

#[cfg(test)]
mod tests {
    use crate::filmic::*;

    #[test]
    fn filmic() {
        0f32.filmic();
        [0f32, 0f32, 0f32].filmic();
        #[cfg(feature = "glam_support")]
        glam::Vec3::zero().filmic();
    }
}
