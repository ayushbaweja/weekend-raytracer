use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::DivAssign;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    // default constructor
    pub fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0]}
    }

    // contructor with values
    pub fn new_with_values(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Self { e: [e0, e1, e2]}
    }

    // getter
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    // getter
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    // getter
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
 
    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }
 
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }
 
    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        Vec3 {
            e: [
                self.e[0] / length,
                self.e[1] / length,
                self.e[2] / length,
            ],
        }
    }

    pub fn add_scalar(&self, scalar: f64) -> Vec3 {
        Vec3 {
          e: [
            self.e[0] + scalar,
            self.e[1] + scalar,
            self.e[2] + scalar,
          ]
        }
       }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}
  
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.mul_assign(1.0 / rhs);
    }
}

// Alias
pub type Point3 = Vec3;

// Vec3 Utility Functions

use std::fmt;

impl fmt::Display for Vec3 {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
   }
}

impl std::ops::Add for Vec3 {
   type Output = Vec3;

   fn add(self, other: Vec3) -> Vec3 {
       Vec3 {
           e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]],
       }
   }
}

impl std::ops::Sub for Vec3 {
   type Output = Vec3;

   fn sub(self, other: Vec3) -> Vec3 {
       Vec3 {
           e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]],
       }
   }
}

impl std::ops::Mul<Vec3> for Vec3 {
   type Output = Vec3;

   fn mul(self, other: Vec3) -> Vec3 {
       Vec3 {
           e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]],
       }
   }
}

impl std::ops::Mul<f64> for Vec3 {
   type Output = Vec3;

   fn mul(self, t: f64) -> Vec3 {
       Vec3 {
           e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
       }
   }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        vec * self
    }
}

impl std::ops::Div<f64> for Vec3 {
   type Output = Vec3;

   fn div(self, t: f64) -> Vec3 {
       self * (1.0 / t)
   }
}
