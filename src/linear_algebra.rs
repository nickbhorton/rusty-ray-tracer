pub mod linear_algebra {
    pub struct Vec4<T>
    where
        T: Copy,
    {
        pub x: T,
        pub y: T,
        pub z: T,
        pub w: T,
    }

    impl<T> Vec4<T>
    where
        T: Copy,
    {
        pub fn from(x: T, y: T, z: T, w: T) -> Vec4<T> {
            Vec4 { x, y, z, w }
        }
    }

    pub struct Vec3<T>
    where
        T: Copy,
    {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl<T> Vec3<T>
    where
        T: Copy,
    {
        pub fn from(x: T, y: T, z: T) -> Vec3<T> {
            Vec3 { x, y, z }
        }
    }

    pub struct Vec2<T>
    where
        T: Copy,
    {
        pub x: T,
        pub y: T,
    }

    impl<T> Vec2<T>
    where
        T: Copy,
    {
        pub fn from(x: T, y: T) -> Vec2<T> {
            Vec2 { x, y }
        }
    }
}
