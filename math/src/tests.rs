#[cfg(test)]
mod tests_vec2 {
    use crate::vec::Vec2;

    #[test]
    fn new() {
        let vec = Vec2::new(123., 0.001);
        assert_eq!(vec.x, 123.);
        assert_eq!(vec.y, 0.001);
    }

    #[test]
    fn length() {
        let vec = Vec2::default();
        assert_eq!(vec.length(), 0.);

        let vec = Vec2::new(2., 0.);
        assert_eq!(vec.length(), 2.);

        let vec = Vec2::new(-10., 24.);
        assert_eq!(vec.length(), 26.);
    }

    #[test]
    fn add() {
        let vec = Vec2::default();
        let rhs = Vec2::new(1., 2.);
        assert_eq!(vec + rhs, Vec2::new(1., 2.));

        let vec = Vec2::new(2., 1.);
        let rhs = Vec2::new(1., 2.);
        assert_eq!(vec + rhs, Vec2::new(3., 3.));
    }

    #[test]
    fn add_assign() {
        let mut vec = Vec2::default();
        vec += Vec2::new(1., 2.);
        assert_eq!(vec, Vec2::new(1., 2.));

        let mut vec = Vec2::new(2., 1.);
        vec += Vec2::new(1., 2.);
        assert_eq!(vec, Vec2::new(3., 3.));
    }

    #[test]
    fn sub() {
        let vec = Vec2::default();
        let rhs = Vec2::new(1., 2.);
        assert_eq!(vec - rhs, Vec2::new(-1., -2.));

        let vec = Vec2::new(2., 1.);
        let rhs = Vec2::new(1., 2.);
        assert_eq!(vec - rhs, Vec2::new(1., -1.));
    }

    #[test]
    fn sub_assign() {
        let mut vec = Vec2::default();
        vec -= Vec2::new(1., 2.);
        assert_eq!(vec, Vec2::new(-1., -2.));

        let mut vec = Vec2::new(2., 1.);
        vec -= Vec2::new(1., 2.);
        assert_eq!(vec, Vec2::new(1., -1.));
    }
}

#[cfg(test)]
mod tests_vec3 {
    use crate::vec::Vec3;

    #[test]
    fn new() {
        let vec = Vec3::new(123., 0.001, 1.);
        assert_eq!(vec.x, 123.);
        assert_eq!(vec.y, 0.001);
        assert_eq!(vec.z, 1.);
    }

    #[test]
    fn length() {
        let vec = Vec3::default();
        assert_eq!(vec.length(), 0.);

        let vec = Vec3::new(2., 0., 0.);
        assert_eq!(vec.length(), 2.);

        let vec = Vec3::new(-10., 24., 0.);
        assert_eq!(vec.length(), 26.);
    }

    #[test]
    fn add() {
        let vec = Vec3::default();
        let rhs = Vec3::new(1., 2., 3.);
        assert_eq!(vec + rhs, Vec3::new(1., 2., 3.));

        let vec = Vec3::new(3., 2., 1.);
        let rhs = Vec3::new(1., 2., 3.);
        assert_eq!(vec + rhs, Vec3::new(4., 4., 4.));
    }

    #[test]
    fn add_assign() {
        let mut vec = Vec3::default();
        vec += Vec3::new(1., 2., 3.);
        assert_eq!(vec, Vec3::new(1., 2., 3.));

        let mut vec = Vec3::new(3., 2., 1.);
        vec += Vec3::new(1., 2., 3.);
        assert_eq!(vec, Vec3::new(4., 4., 4.));
    }

    #[test]
    fn sub() {
        let vec = Vec3::default();
        let rhs = Vec3::new(1., 2., 3.);
        assert_eq!(vec - rhs, Vec3::new(-1., -2., -3.));

        let vec = Vec3::new(3., 2., 1.);
        let rhs = Vec3::new(1., 2., 3.);
        assert_eq!(vec - rhs, Vec3::new(2., 0., -2.));
    }

    #[test]
    fn sub_assign() {
        let mut vec = Vec3::default();
        vec -= Vec3::new(1., 2., 3.);
        assert_eq!(vec, Vec3::new(-1., -2., -3.));

        let mut vec = Vec3::new(3., 2., 1.);
        vec -= Vec3::new(1., 2., 3.);
        assert_eq!(vec, Vec3::new(2., 0., -2.));
    }

    #[test]
    fn cross_product() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(2., 1., -2.);
        assert_eq!(a.cross_product(b), Vec3::new(-7., 8., -3.));

        let a = Vec3::new(-1., 2., -2.);
        let b = Vec3::new(2., 1., -1.);
        assert_eq!(a.cross_product(b), Vec3::new(0., -5., -5.));
    }
}

#[cfg(test)]
mod tests_vec4 {
    use crate::vec::Vec4;

    #[test]
    fn new() {
        let vec = Vec4::new(123., 0.001, 1., 0.);
        assert_eq!(vec.x, 123.);
        assert_eq!(vec.y, 0.001);
        assert_eq!(vec.z, 1.);
        assert_eq!(vec.w, 0.);
    }

    #[test]
    fn length() {
        let vec = Vec4::default();
        assert_eq!(vec.length(), 0.);

        let vec = Vec4::new(2., 0., 0., 0.);
        assert_eq!(vec.length(), 2.);

        let vec = Vec4::new(-10., 24., 0., 0.);
        assert_eq!(vec.length(), 26.);
    }

    #[test]
    fn add() {
        let vec = Vec4::default();
        let rhs = Vec4::new(1., 2., 3., 4.);
        assert_eq!(vec + rhs, Vec4::new(1., 2., 3., 4.));

        let vec = Vec4::new(4., 3., 2., 1.);
        let rhs = Vec4::new(1., 2., 3., 4.);
        assert_eq!(vec + rhs, Vec4::new(5., 5., 5., 5.));
    }

    #[test]
    fn add_assign() {
        let mut vec = Vec4::default();
        vec += Vec4::new(1., 2., 3., 4.);
        assert_eq!(vec, Vec4::new(1., 2., 3., 4.));

        let mut vec = Vec4::new(4., 3., 2., 1.);
        vec += Vec4::new(1., 2., 3., 4.);
        assert_eq!(vec, Vec4::new(5., 5., 5., 5.));
    }

    #[test]
    fn sub() {
        let vec = Vec4::default();
        let rhs = Vec4::new(1., 2., 3., 4.);
        assert_eq!(vec - rhs, Vec4::new(-1., -2., -3., -4.));

        let vec = Vec4::new(4., 3., 2., 1.);
        let rhs = Vec4::new(1., 2., 3., 4.);
        assert_eq!(vec - rhs, Vec4::new(3., 1., -1., -3.));
    }

    #[test]
    fn sub_assign() {
        let mut vec = Vec4::default();
        vec -= Vec4::new(1., 2., 3., 4.);
        assert_eq!(vec, Vec4::new(-1., -2., -3., -4.));

        let mut vec = Vec4::new(4., 3., 2., 1.);
        vec -= Vec4::new(1., 2., 3., 4.);
        assert_eq!(vec, Vec4::new(3., 1., -1., -3.));
    }
}
