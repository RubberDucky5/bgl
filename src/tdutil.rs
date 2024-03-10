#![allow(dead_code)]

extern crate sdl2;
use sdl2::{rect, render::RenderTarget};

#[derive(Copy, Clone)]
pub struct Camera {
    pub pos: Point,
    pub res: rect::Point,
    pub fov: i32,
}

impl Camera {
    pub fn new (pos: Point, res: rect::Point, fov: i32) -> Self {
        Self {
            pos, res, fov,
        }
    }

    pub fn point_to_ss (self, point: Point) -> rect::Point {
        rect::Point::new((  point.x / 1.0 ) as i32 + self.res.x / 2,
                         ( point.y / 1.0 ) as i32 + self.res.y / 2)
    }

    pub fn tri_to_ss (self, tri: Tri) -> [rect::Point; 3] {
        let mut out: [rect::Point; 3] = [rect::Point::new(0,0); 3];
        out[0] = tri.a.project(self);
        out[1] = tri.b.project(self);
        out[2] = tri.c.project(self);
        out
    }

    pub fn render<T: RenderTarget> (self, canvas: &mut sdl2::render::Canvas<T>, geometry: &Vec<Geometry>) {
        for g in geometry.iter() {
            for t in g.tris.iter() {
                let p = self.tri_to_ss(**t);
                canvas.draw_line(p[0], p[1]);
                canvas.draw_line(p[1], p[2]);
                canvas.draw_line(p[2], p[0]);
            }
        }
    }
}

#[derive(Clone)]
pub struct Geometry<'a> {
    pub pos: Point,
    pub tris: Vec<&'a Tri>,
}

impl<'a> Geometry<'a> {
    pub fn new (pos: Point) -> Geometry<'a> {
        Self {
            pos,
            tris: Vec::new(),
        }
    }

    // pub fn rot_x (&mut self, a: f32) {
    //     for t in self.tris.iter() {
    //         t.rot_x(a);
    //     }
    // }

    // pub fn rot_y (&mut self, a: f32) {
    //     for t in self.tris.iter() {
    //         t.rot_y(a);
    //     }
    // }

    // pub fn rot_z (&mut self, a: f32) {
    //     for t in self.tris.iter() {
    //         t.rot_z(a);
    //     }
    // } 

    pub fn add_tri (&mut self, tri: &'a Tri) {
        self.tris.push(tri);
    }
}

#[derive(Copy, Clone)]
pub struct Tri {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Tri {
    pub fn new (a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c, }
    }

    pub fn rot_x (&mut self, a: f32) {
        self.a.rot_x(a);
        self.b.rot_x(a);
        self.c.rot_x(a);
    }

    pub fn rot_y (&mut self, a: f32) {
        self.a.rot_y(a);
        self.b.rot_y(a);
        self.c.rot_y(a);
    }

    pub fn rot_z (&mut self, a: f32) {
        self.a.rot_z(a);
        self.b.rot_z(a);
        self.c.rot_z(a);
    }

    pub fn project (self, camera: Camera) -> [rect::Point; 3] {
        let mut out: [rect::Point; 3] = [rect::Point::new(0,0); 3];
        out[0] = self.a.project(camera);
        out[1] = self.b.project(camera);
        out[2] = self.c.project(camera);
        out
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Point {
    pub fn new (x : f32, y : f32, z : f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn project (self, camera: Camera) -> rect::Point {
        rect::Point::new((  self.x / 1.0 ) as i32 + camera.res.x / 2,
                         ( self.y / 1.0 ) as i32 + camera.res.y / 2)
    }

    pub fn rot_x (&mut self, a: f32) {
        let mut axis = arr(&[[self.y, self.z]]);
        let c = a.cos();
        let s = a.sin();
        let mat = arr(&[[c, -s],
                                [s,  c]]);
        axis = axis.dot(&mat);

        self.y = axis.get(vec![0,0]);
        self.z = axis.get(vec![1,0]);
    }

    pub fn rot_y (&mut self, a: f32) {
        let mut axis = arr(&[[self.x, self.z]]);
        let c = a.cos();
        let s = a.sin();
        let mat = arr(&[[c, -s],
                                [s,  c]]);
        axis = axis.dot(&mat);

        self.x = axis.get(vec![0,0]);
        self.z = axis.get(vec![1,0]);
    }

    pub fn rot_z (&mut self, a: f32) {
        let mut axis = arr(&[[self.y, self.x]]);
        let c = a.cos();
        let s = a.sin();
        let mat = arr(&[[c, -s],
                                [s,  c]]);
        axis = axis.dot(&mat);

        self.y = axis.get(vec![0,0]);
        self.x = axis.get(vec![1,0]);
    }

    pub fn length (self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn dot (self, other: Self) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
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
}

impl ToString for Point {
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

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul for Point {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Div for Point {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

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
    pub fn from_arr<Mat: AsRef<[Row]>, Row: AsRef<[f32]>>(arr: &Mat) -> Self {
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
               m.set(vec![x,y], func(vec![x,y]));
            }
        }

        m
    }

    pub fn get(&self, p: Vec<usize>) -> f32 {
        self.values[p[1]][p[0]]
    }

    pub fn set(&mut self, p: Vec<usize>, v: f32) {
        self.values[p[1]][p[0]] = v;
    }

    pub fn transpose (self) -> Self {
      Matrix::from_fn(vec![self.size[1], self.size[0]], |p| self.get(vec![p[1], p[0]]))
    }

    pub fn dot(&self, other: &Self) -> Self {
        // assert_eq!(self.size[1], other.size[0]);

        let mut out = Matrix::new(vec![other.size[0], self.size[1]]);

        for y1 in 0..self.size[1] {
            for x2 in 0..other.size[0] {
                let mut sum: f32 = 0.0;
                for x1 in 0..self.size[0] {
                    sum += self.get(vec![x1, y1]) * other.get(vec![x2, x1]);
                }
                out.set(vec![x2, y1], sum);
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
                out.push_str(self.get(vec![x, y]).to_string().as_str());
                out.push_str("\t\t");
            }
            out.push('\n');
        }

        out
    }
}

pub fn arr<Mat: AsRef<[Row]>, Row: AsRef<[f32]>>(a: &Mat) -> Matrix {
    Matrix::from_arr(a)
}
