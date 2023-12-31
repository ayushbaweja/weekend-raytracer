use super::vec3::{Vec3, Point3};

#[derive(Clone, Copy)]
pub struct Ray {
    orig : Point3,
    dir : Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray {
            orig,
            dir,
        }
    }

    pub fn orig(&self) -> Point3 {
        self.orig
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

