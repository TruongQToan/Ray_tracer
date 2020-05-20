use crate::utilities::clamp;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[allow(dead_code)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[allow(dead_code)]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[allow(dead_code)]
    pub fn z(&self) -> f64 {
        self.z
    }

    #[allow(dead_code)]
    pub fn l2(&self) -> f64 {
        self.l2_squared().sqrt()
    }

    #[allow(dead_code)]
    pub fn l2_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, t: Vec3) -> Vec3 { Vec3::new(self * t.x, self * t.y, self * t.z) }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}


#[allow(dead_code)]
pub fn dot(_lhs: &Vec3, _rhs: &Vec3) -> f64 {
    _lhs.x * _rhs.x + _lhs.y * _rhs.y + _lhs.z * _rhs.z
}

#[allow(dead_code)]
pub fn cross(_lhs: Vec3, _rhs: Vec3) -> Vec3 {
    Vec3::new(_lhs.y * _rhs.z - _lhs.z * _rhs.y, _lhs.z * _rhs.x - _lhs.x * _rhs.z, _lhs.x * _rhs.y - _lhs.y * _rhs.x)
}

#[allow(dead_code)]
pub fn unit_vector(v: Vec3) -> Vec3 {
    let l = v.l2();
    Vec3::new(v.x / l, v.y / l, v.z / l)
}

pub type Color = Vec3;

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let samples_per_pixel: i32 = 100;
        let mut r: f64 = self.x;
        let mut g: f64 = self.y;
        let mut b: f64 = self.z;
        
        let scale: f64 = 1.0 / (samples_per_pixel as f64);
        r *= scale;
        g *= scale;
        b *= scale;
        
        let ir: u8 = ((256.0 * clamp(r, 0.0, 0.999)).floor()) as u8;
        let ig: u8 = ((256.0 * clamp(g, 0.0, 0.999)).floor()) as u8;
        let ib: u8 = ((256.0 * clamp(b, 0.0, 0.999)).floor()) as u8;
        write!(f, "{} {} {}\n", ir, ig, ib)
    }
}

pub type Point3 = Vec3;
