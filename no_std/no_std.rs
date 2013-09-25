#[no_std];
#[allow(cstack)];
#[allow(ctypes)];

#[abi = "cdecl"]
extern {
    fn rand() -> i32;
    fn write(fd: i32, buf: *u8, count: uint) -> int;
}

extern "rust-intrinsic" {
    fn transmute<T,S>(e: T) -> S;
    fn sqrtf32(x: f32) -> f32;
    fn powf32(x: f32, y: f32) -> f32;
    fn ceilf32(x: f32) -> f32;
    pub fn offset<T>(dst: *T, offset: int) -> *T;
}

pub struct Vec {
    x: f32,
    y: f32,
    z: f32
}

fn V(x: f32, y: f32, z: f32) -> Vec {
    Vec { x: x, y: y, z: z }
}
fn add(v: Vec, w: Vec) -> Vec {
    V(v.x + w.x, v.y + w.y, v.z + w.z)
}
fn mul(v: Vec, scale: f32) -> Vec {
    V(v.x * scale, v.y * scale, v.z * scale)
}
pub fn dot(v: Vec, w: Vec) -> f32 {
    v.x * w.x + v.y * w.y + v.z * w.z
}

fn cross(v: Vec, w: Vec) -> Vec {
    V(v.y * w.z - v.z * w.y,
      v.z * w.x - v.x * w.z,
      v.x * w.y - v.y * w.x)
}
fn normalise(v: Vec) -> Vec {
    mul(v, (1. / unsafe { sqrtf32(dot(v, v)) }))
}

static G: &'static [i32] = &[301252, 336932, 402628, 468255, 304324, 458756, 0, 0, 0];

fn print(v: &[u8]) {
    unsafe {
        let (ptr, len): (*u8, uint) = transmute(v);
        write(1, ptr, len);
    }
}
fn index(v: &[i32], i: uint) -> i32 {
    unsafe {
        let (ptr, _len): (*i32, uint) = transmute(v);

        *offset(ptr, i as int)
    }
}

fn R() -> f32 {
    unsafe { rand() as f32 / 2147483647. }
}

/*
&[0b0000000000000000000,
  0b0000000000000000000,
  0b0000000000000000000,
  0b1110000000000000100,
  0b1001010010011000100,
  0b1110010010100011111,
  0b1100010010011000100,
  0b1010010010000100100,
  0b1001001100011000100];
*/

fn T(o: Vec, d: Vec, t: &mut f32, n: &mut Vec) -> i32 {
    *t = 1e9;
    let mut m = 0;
    let p = -o.z / d.z;
    *n = V(0.,0.,0.);
    if (p > 0.01) {
        *t = p;
        *n = V(0., 0., 1.);
        m = 1;
    }
    let mut k = 18i;
    while k >= 0 {
        let mut j = 8i;
        while j >= 0 {
            if index(G, j as uint) & (1 << k) != 0 {
                let p = add(o, V(-k as f32, 3., -(j as f32) - 4.));
                let (b, c) = (dot(p, d), dot(p, p) - 1.);
                let q = b * b - c;

                if q > 0. {
                    let s = -b - unsafe {sqrtf32(q)};
                    if 0.01 < s && s < *t {
                        *t = s;
                        *n = normalise(add(p, mul(d, *t)));
                        m = 2;
                    }
                }
            }
            j -= 1;
        }
        k -= 1;
    }

    m
}

fn S(o: Vec, d: Vec) -> Vec {
    let mut t = 0.;
    let mut n = V(0.,0.,0.);
    let m = T(o, d, &mut t, &mut n);

    if m == 0 {
        return mul(V(0.7, 0.6, 1.), unsafe {powf32(1. - d.z, 4.)});
    }

    let h = add(o, mul(d, t));
    let l = normalise(add(V(9. + R(), 9. + R(), 16.), mul(h, -1.)));
    let r = add(d, mul(n, dot(n, d) * -2.));
    let mut b = dot(l, n);

    if b < 0. || T(h, l, &mut t, &mut n) != 0  {
        b = 0.;
    }

    let p = unsafe {powf32(dot(l,r) * (b > 0.) as f32, 99.)};
    if m == 1 {
        let h_ = mul(h, 0.2);
        mul(if unsafe {ceilf32(h_.x) + ceilf32(h_.y)} as i32 & 1 == 1 {
            V(3., 1., 1.)
        } else {
            V(3., 3., 3.)
        }, (b * 0.2 + 0.1))
    } else {
        add(V(p, p, p), mul(S(h, r), 0.5))
    }
}

#[fixed_stack_segment]
#[start]
fn main(_: int, _: **u8) -> int {
    print(bytes!("P6 512 512 255 "));

    let g = normalise(V(-5.5, -16., 0.));
    let a = mul(normalise(cross(V(0., 0., 1.), g)), 0.002);
    let b = mul(normalise(cross(g, a)), 0.002);
    let c = add(mul(add(a, b), -256.), g);

    let mut y = 511;
    while y >= 0 {
        let mut x = 511;
        while x >= 0 {
            let (x_, y_) = (x as f32, y as f32);
            let mut p = V(13., 13., 13.);

            let mut r = 63;
            while r >= 0 {
                let t = add(mul(a, (R() - 0.5) * 99.), mul(b, (R() - 0.5) * 99.));
                let dir = normalise(add(mul(t, -1.),
                              mul(add(mul(a, (x_ + R())),
                                  add(mul(b, (y_ + R())),
                                      c)), 16.)));
                p = add(mul(S(add(V(17., 16., 8.), t), dir), 3.5), p);

                r-= 1;
            }
            print([p.x as u8, p.y as u8, p.z as u8]);

            x -= 1;
        }
        y -= 1
    }
    0
}
