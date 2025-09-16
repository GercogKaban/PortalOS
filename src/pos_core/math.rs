use core::sync::atomic;

pub struct Vec2<T>
{
    x: T,
    y: T,
}

pub struct Vec3<T>
{
    x: T,
    y: T,
    z: T,
}

pub struct Vec4<T>
{
    x: T,
    y: T,
    z: T,
    w: T,
}

pub type FVec2 = Vec2<f32>;
pub type FVec3 = Vec3<f32>;
pub type FVec4 = Vec4<f32>;

pub type IVec2 = Vec2<i32>;
pub type UVec2 = Vec2<u32>;
pub type IVec3 = Vec3<i32>;
pub type UVec3 = Vec3<u32>;
pub type IVec4 = Vec4<i32>;
pub type UVec4 = Vec4<u32>;

pub type IVec2A = Vec2<atomic::AtomicI32>;
pub type UVec2A = Vec2<atomic::AtomicU32>;
pub type IVec3A = Vec3<atomic::AtomicI32>;
pub type UVec3A = Vec3<atomic::AtomicU32>;
pub type IVec4A = Vec4<atomic::AtomicI32>;
pub type UVec4A = Vec4<atomic::AtomicU32>;