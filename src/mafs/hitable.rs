 use super::vec::Vec3;
 use super::ray::Ray;

#[derive(Copy, Clone, Debug)]
 pub struct HitRecord{
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
 }

 impl HitRecord{
    pub fn new() -> HitRecord{
        return HitRecord { t: 0.0, 
                           p: Vec3::new(0.0,0.0,0.0), normal: Vec3::new(0.0,0.0,0.0) };
    }
 }
 pub trait Hitable{
    fn hit(&self, t: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
 }