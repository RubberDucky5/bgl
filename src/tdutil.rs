#![allow(dead_code)]

extern crate sdl2;
use sdl2::{rect, render::RenderTarget};

#[derive(Clone)]
pub struct Camera {
    pub res: rect::Point,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    proj_mat: Matrix,
    pub transform: Transform,
}

impl Camera {
    pub fn new (pos: Vec3, res: rect::Point, fov: f32) -> Self {
        let mut out = Self {
            res,
            fov,
            near: 0.1,
            far: 1000.0,
            proj_mat: Matrix::new(vec![4,4]),
            transform: Transform::new(),
        };

        out.calculate_projection_matrix();

        out
    }

    fn calculate_projection_matrix (&mut self) {
        let ar: f32 = (self.res.x / self.res.y) as f32;
        let vfov = (self.fov/2.).tan();

        self.proj_mat = arr([
            [1./(ar*vfov), 0., 0., 0.],
            [0., 1./vfov, 0., 0.],
            [0., 0., (-self.near-self.far)/(self.near-self.far), (2. * self.far * self.near)/(self.near-self.far)],
            [0., 0., 1., 0.]
        ]);
    }

    pub fn point_to_ss (&self, point: &Vec3) -> rect::Point {
        let out = self.proj_mat.dot(&arr([[point.x, point.y, point.z, 1.]]).transpose());
        let out = rect::Point::new(
                (((out.get(&vec![0,0]) / out.get(&vec![0,3]) + 1.) / 2.) * self.res.y as f32) as i32,
                (((out.get(&vec![0,1]) / out.get(&vec![0,3]) + 1.) / 2.) * self.res.y as f32) as i32
        );

        out
    }

    pub fn tri_to_ss (&self, tri: &Tri) -> [rect::Point; 3] {
        let mut out: [rect::Point; 3] = [rect::Point::new(0,0); 3];
        out[0] = self.point_to_ss(&tri.a);
        out[1] = self.point_to_ss(&tri.b);
        out[2] = self.point_to_ss(&tri.c);
        out
    }

    pub fn render<T: RenderTarget> (&self, canvas: &mut sdl2::render::Canvas<T>, geometry: &Vec<Geometry>) {
        let look_dir = Vec3::new(0.,0.,1.);
        let look_dir = self.transform.apply_to_vector(&look_dir);
        for g in geometry.iter() {
            let tris = g.apply_transform();
            for t in tris.iter() {
                if t.should_backface_cull(look_dir) {
                    continue;
                }
                let p = self.tri_to_ss(t);
                let _ = (canvas.draw_line(p[0], p[1]),
                canvas.draw_line(p[1], p[2]),
                canvas.draw_line(p[2], p[0]));
            }
        }
    }
}

#[derive(Clone)]
pub struct Geometry {
    pub tris: Vec<Tri>,
    pub transform: Transform,
}

impl Geometry {
    pub fn new () -> Geometry {
        Self {
            tris: Vec::new(),
            transform: Transform::new(),
        }
    }

    pub fn apply_transform (&self) -> Vec<Tri> {
        let mut out = self.tris.clone();
        
        for tri in out.iter_mut() {
            tri.a = self.transform.apply_to_vector(&tri.a);
            tri.b = self.transform.apply_to_vector(&tri.b);
            tri.c = self.transform.apply_to_vector(&tri.c);
        }

        out
    }

    pub fn add_tri (&mut self, tri: Tri) {
        self.tris.push(tri);
    }

    pub fn add_tris (&mut self, tri: Vec<Tri>) {
        for t in tri.iter() {
            self.tris.push(*t);
        }
    }
}

#[derive(Clone)]
pub struct Transform {
    pub mat: Matrix
}

impl Transform {
    pub fn new () -> Self {
        Self {
            mat: arr([
                [1.,0.,0.,0.],
                [0.,1.,0.,0.],
                [0.,0.,1.,0.],
                [0.,0.,0.,1.]
                ])
        }
    }

    pub fn translate (&mut self, v: Vec3) {
        let delta = arr([
            [0.,0.,0.,v.x],
            [0.,0.,0.,v.y],
            [0.,0.,0.,v.z],
            [0.,0.,0.,0.]
        ]);

        self.mat = self.mat.add(&delta);
    }

    pub fn rot_x (&mut self, a: f32) {
        let c = a.cos();
        let s = a.sin();

        let delta = arr([
            [1.,0.,0.,0.],
            [0.,c,-s,0.],
            [0.,s,c,0.],
            [0.,0.,0.,1.]
        ]);

        self.mat = self.mat.dot(&delta);
    }

    pub fn rot_y (&mut self, a: f32) {
        let c = a.cos();
        let s = a.sin();

        let delta = arr([
            [c,0.,s,0.],
            [0.,1.,0.,0.],
            [-s,0.,c,0.],
            [0.,0.,0.,1.]
        ]);

        self.mat = self.mat.dot(&delta);
    }

    pub fn rot_z (&mut self, a: f32) {
        let c = a.cos();
        let s = a.sin();

        let delta = arr([
            [c,-s,0.,0.],
            [s,c,0.,0.],
            [0.,0.,1.,0.],
            [0.,0.,0.,1.]
        ]);

        self.mat = self.mat.dot(&delta);
    }

    pub fn set_pos (&mut self, v: Vec3) {
        

        self.mat.set(&vec![3,0], v.x);
        self.mat.set(&vec![3,1], v.y);
        self.mat.set(&vec![3,2], v.z);
    }

    pub fn apply_to_vector (&self, p: &Vec3) -> Vec3 {
        let p = arr([[p.x, p.y, p.z, 1.]]).transpose();

        let out = self.mat.dot(&p);
        let out = Vec3::new(out.get(&vec![0,0]), out.get(&vec![0,1]), out.get(&vec![0,2]));

        out
    }
}

#[derive(Copy, Clone)]
pub struct Tri {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Tri {
    pub fn new (a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c, }
    }

    pub fn get_normal (&self) -> Vec3 {
        let a = self.a - self.b;
        let b = self.c - self.b;

        a.cross(&b).normalize()
    }

    pub fn should_backface_cull (&self, vec: Vec3) -> bool {
        self.get_normal().dot(vec) < 0.1 // Why is this 0.1 and not 0???????????????
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Vec3 {
    pub fn new (x : f32, y : f32, z : f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn zero () -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn length (self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn dot (self, other: Self) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross (&self, other: &Self) -> Self{
        Vec3::new(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x
        )
    }

    pub fn add (mut self, other: Self) -> Self {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn sub (mut self, other: Self) -> Self {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn mul (mut self, other: Self) -> Self {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn div (mut self, other: Self) -> Self {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn normalize (&mut self) -> Self {
        let l = self.length();

        self.x /= l;
        self.y /= l;
        self.z /= l;

        *self
    }
}

impl ToString for Vec3 {
    fn to_string (&self) -> String {
        let mut out = String::from("( ");
        out.push_str(self.x.to_string().as_str());
        out.push_str(", ");
        out.push_str(self.y.to_string().as_str());
        out.push_str(", ");
        out.push_str(self.z.to_string().as_str());
        out.push_str(" )");
        out
    } 
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}


// TODO: Potential Optimization here: Use static size arrays, 4x4 and 1x4

#[derive(Clone, Debug)]
pub struct Matrix {
    values: Vec<Vec<f32>>,
    size: Vec<usize>,
}

impl Matrix {
    pub fn new(size: Vec<usize>) -> Self {
        let x: usize = match size.get(0) {
            Some(o) => *o,
            None => panic!("Expected Vector of length 2 for size"),
        };

        let y: usize = match size.get(1) {
            Some(o) => *o,
            None => panic!("Expected Vector of length 2 for size"),
        };

        Self {
            values: vec![vec![0.0; x + 1]; y + 1],
            size: size.clone(),
        }
    }
    pub fn from_arr<Mat: AsRef<[Row]>, Row: AsRef<[f32]>>(arr: Mat) -> Self {
        let s0: usize = arr.as_ref().len();
        let s1: usize = arr.as_ref()[0].as_ref().len();
      
        let s: Vec<usize> = vec![s1, s0];

        let mut v: Vec<Vec<f32>> = vec![vec![0.0; s1]; s0];

        let (mut x, mut y) = (0, 0);
        for j in arr.as_ref() {
            for i in j.as_ref() {
                v[y][x] = *i;
                x += 1;
            }
            x = 0;
            y += 1;
        }

        Self { values: v, size: s }
    }

    pub fn from_fn<T: Fn(Vec<usize>) -> f32>(size: Vec<usize>, func: T ) -> Self {
        let mut m = Matrix::new(vec![size[0], size[1]]);
        
        for x in 0..size[0] {
            for y in 0..size[1] {
               m.set(&vec![x,y], func(vec![x,y]));
            }
        }

        m
    }

    pub fn get(&self, p: &Vec<usize>) -> f32 {
        self.values[p[1]][p[0]]
    }

    pub fn set(&mut self, p: &Vec<usize>, v: f32) {
        self.values[p[1]][p[0]] = v;
    }

    pub fn add (&self, other: &Matrix) -> Self {
        assert_eq!(self.size, other.size);
        
        let out = Matrix::from_fn(self.size.clone(), |c| self.get(&c)+other.get(&c));

        out
    }

    pub fn transpose (self) -> Self {
      Matrix::from_fn(vec![self.size[1], self.size[0]], |p| self.get(&vec![p[1], p[0]]))
    }

    pub fn dot(&self, other: &Self) -> Self {
        let mut out = Matrix::new(vec![other.size[0], self.size[1]]);

        for y1 in 0..self.size[1] {
            for x2 in 0..other.size[0] {
                let mut sum: f32 = 0.0;
                for x1 in 0..self.size[0] {
                    sum += self.get(&vec![x1, y1]) * other.get(&vec![x2, x1]);
                }
                out.set(&vec![x2, y1], sum);
            }
        }

        out
    }
}

impl ToString for Matrix {
    fn to_string(&self) -> String {
        let mut out = String::from("");
        out.push_str(format!("{:?}\n", self.size).as_str());

        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                out.push_str(self.get(&vec![x, y]).to_string().as_str());
                out.push_str("\t\t");
            }
            out.push('\n');
        }

        out
    }
}

pub fn arr<Mat: AsRef<[Row]>, Row: AsRef<[f32]>>(a: Mat) -> Matrix {
    Matrix::from_arr(a)
}
