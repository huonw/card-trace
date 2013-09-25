#include <stdlib.h>
#include <math.h>
#include <unistd.h>
typedef int i32;
typedef float f32;

typedef struct {
  f32 x,y,z;
} Vec;

Vec V(f32 x, f32 y, f32 z) {
	Vec r = {.x = x, .y = y, .z = z};
	return r;
}
Vec add(Vec v, Vec w) {
    return V(v.x + w.x, v.y + w.y, v.z + w.z);
}
Vec mul(Vec v, f32 scale) {
	return V(v.x * scale, v.y * scale, v.z * scale);
}
f32 dot(Vec v, Vec w) {
	return v.x * w.x + v.y * w.y + v.z * w.z;
}
Vec cross(Vec v, Vec w) {
	return V(v.y * w.z - v.z * w.y,
			 v.z * w.x - v.x * w.z,
			 v.x * w.y - v.y * w.x);
}
Vec normalise(Vec v) {
	return mul(v, 1. / sqrtf(dot(v,v)));
}

i32 G[]={301252, 336932, 402628, 468255, 304324, 458756, 0, 0, 0};

void print(char* vec, int len) {
	write(1, vec, len);
}

f32 R() {
	return (f32)rand() / RAND_MAX;
}

i32 T(Vec o ,Vec d, f32* t, Vec* n) {
  *t = 1e9;
  i32 m = 0;
  f32 p = -o.z / d.z;

  if(p > 0.01) {
	  *t = p;
	  *n = V(0., 0., 1.);
	  m = 1;
  }

  for(i32 k=19;k--;) {
	  for(i32 j=9;j--;) {
		  if ((G[j] & (1 << k)) != 0 ) {
			  Vec p = add(o, V(-k, 3., -j - 4.));
			  f32 b = dot(p, d), c = dot(p, p) - 1., q = b * b - c;

			  if (q > 0.) {
				  f32 s = -b - sqrtf(q);

				  if (0.01 < s && s < *t) {
					  *t = s;
					  *n = normalise(add(p, mul(d, *t)));
					  m = 2;
				  }
			  }
		  }
	  }
  }

  return m;
}

Vec S(Vec o, Vec d) {
  f32 t;
  Vec n = V(0., 0., 0.);

  i32 m = T(o, d, &t, &n);

  if(m == 0)
	  return mul(V(0.7, 0.6, 1.), powf(1 - d.z, 4.));

  Vec h = add(o, mul(d, t)),
	  l = normalise(add(V(9. + R(), 9. + R(), 16.), mul(h,-1.))),
	  r = add(d, mul(n, dot(n, d) * -2.));

  f32 b=dot(l, n);

  if (b < 0. || T(h, l, &t, &n) != 0)
    b=0.;

  f32 p = powf(dot(l, r) * (b > 0.), 99.);

  if (m == 1) {
	  Vec h_ = mul(h, 0.2);
	  return mul((((i32)(ceil(h_.x)+ceil(h_.y)) & 1) == 1 ?
				  V(3., 1., 1.)
				  :
				  V(3., 3. ,3.)
				  ), (b * 0.2 + 0.1));
  } else {
	  return add(V(p, p, p), mul(S(h, r), 0.5));
  }
}

i32 main() {
	char buf[] = "P6 512 512 255 ";
	print(buf, sizeof buf);

	Vec g = normalise(V(-5.5, -16., 0.)),
		a = mul(normalise(cross(V(0., 0., 1.), g)), .002),
		b = mul(normalise(cross(g, a)), .002),
		c = add(mul(add(a, b), -256), g);

	for(i32 y=512;y--;)
		for(i32 x=512;x--;) {
			Vec p = V(13., 13., 13.);

			for(i32 r=64;r--;) {
				Vec t = add(mul(a, (R()-.5) * 99.), mul(b, (R()-.5) * 99.));
				Vec dir = normalise(add(mul(t, -1.),
										mul(add(mul(a, x + R()),
												add(mul(b, y + R()),
													c)), 16.)));
				p = add(mul(S(add(V(17., 16., 8.), t), dir), 3.5), p);
			}

			char buf[] = {p.y, p.z, p.x};
			print(buf, 3);
		}
}
