mod filmic;
#[allow(unused)]
use filmic::*;

#[cfg(test)]
mod tests {
    use crate::filmic::*;

    #[test]
    fn filmic() {
        0f32.filmic();
        [0f32, 0f32, 0f32].filmic();
        #[cfg(feature = "glam_support")]
        glam::Vec3::new(0f32, 0f32, 0f32).filmic();
    }
}
