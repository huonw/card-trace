use std::num;
use std::rand::Rng;
use vector::Vec;

mod vector;

static G: &'static [i32] =
&[0b0000000000000000000,
  0b0000000000000000000,
  0b0000000000000000000,
  0b1110000000000000100,
  0b1001010010011000100,
  0b1110010010100011111,
  0b1100010010011000100,
  0b1010010010000100100,
  0b1001001100011000100];

fn test(o: Vec, d: Vec) -> (i32, f32, Vec) {
    let mut t = 1e9;
    let mut m = 0;
    let p = -o.z / d.z;
    let mut n: Vec = num::zero();
    if (p > 0.01) {
        t = p;
        n.z = 1.;
        m = 1;
    }
    for k in range(0, 19).invert() {
        for (j, g) in G.rev_iter().enumerate().invert() {
            if g & (1 << k) != 0 {
                let p = o + Vec::new(-k as f32, 3., -(j as f32) - 4.);
                let (b, c) = (p.dot(&d), p.dot(&p) - 1.);
                let q = b * b - c;

                if q > 0. {
                    let s = -b - num::sqrt(q);
                    if 0.01 < s && s < t {
                        t = s;
                        n = (p + d * t).normalise();
                        m = 2;
                    }
                }
            }
        }
    }

    (m, t, n)
}

fn sample<R: Rng>(o: Vec, d: Vec, rng: &mut R) -> Vec {
    let (m, t, n) = test(o, d);

    if m == 0 {
        return Vec::new(0.7, 0.6, 1.) * num::pow(1. - d.z, 4.);
    }

    let h = o + d * t;
    let l = (Vec::new(9. + rng.gen(), 9. + rng.gen(), 16.) + h * -1.).normalise();
    let r = d + n * (n.dot(&d) * -2.);
    let mut b = l.dot(&n);

    if b < 0. || { let (m, _, _) = test(h, l); m != 0 } {
        b = 0.;
    }

    let p = num::pow(l.dot(&r) * (b > 0.) as f32, 99.);
    if m == 1 {
        let h_ = h * 0.2;
        (if (h_.x.ceil() + h_.y.ceil()) as i32 & 1 == 1 {
            Vec::new(3., 1., 1.)
        } else {
            Vec::new(3., 3., 3.)
        }) * (b * 0.2 + 0.1)
    } else {
        Vec::new(p, p, p) + sample(h, r, rng) * 0.5
    }
}

fn main() {
    print("P6 512 512 255 ");

    let g = Vec::new(-5.5, -16., 0.).normalise();
    let a = Vec::new(0., 0., 1.).cross(&g).normalise() * 0.002;
    let b = g.cross(&a).normalise() * 0.002;
    let c = (a + b) * -256. + g;

    let mut rng = std::rand::weak_rng();

    for y in range(0, 512).invert() {
        for x in range(0, 512).invert() {
            let (x, y) = (x as f32, y as f32);
            let mut p = Vec::new(13., 13., 13.);

            for _ in range(0, 64).invert() {
                let t = a * (rng.gen::<f32>() - 0.5) * 99. + b * (rng.gen::<f32>() - 0.5) * 99.;
                let dir = (t * -1. +
                           (a * (x + rng.gen()) +
                            b * (y + rng.gen()) + c) * 16.).normalise();
                p = sample(Vec::new(17., 16., 8.) + t, dir, &mut rng) * 3.5 + p;
            }
            std::io::stdout().write([p.x as u8, p.y as u8, p.z as u8]);
        }
    }
}
