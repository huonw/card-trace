use P=std::num::pow,R=std::rand::random;type f=f32;type i=i32;struct V{x:f,y:f,z:f}impl Add<V,V>for V{fn add(&self,o:&V)->V{v(self.x+o.x,self.y+o.y,self.z+o.z)}}impl Mul<f,V>for V{fn mul(&self,&s:&f)->V{v(self.x*s,self.y*s,self.z*s)}}fn v(x:f,y:f,z:f)->V{V{x:x,y:y,z:z}}fn i(w:V,o:V)->f{w.x*o.x+w.y*o.y+w.z*o.z}fn c(w:V,o:V)->V{v(w.y*o.z-w.z*o.y,w.z*o.x-w.x*o.z,w.x*o.y-w.y*o.x)}fn u(w:V)->V{w*(1./i(w,w).sqrt())}fn T(o:V,d:V)->(i,f,V){let G=[0,0,0,458756,304324,468255,402628,336932,301252];let mut t=1e9;let mut m=0;let p=-o.z/d.z;let mut n=v(0.,0.,0.);if p>0.01{t=p;n.z=1.;m=1;}for k in range(0,19){for(j,&g)in G.iter().enumerate(){if 1<<k&g!=0{let p=o+v(-k as f,3.,j as f-12.);let b=i(p,d);let q=b*b-i(p,p)+1.;if q>0.{let s=-b-q.sqrt();if 0.01<s&&s<t{t=s;n=u(p+d*t);m=2}}}}}(m,t,n)}fn S(o:V,d:V)->V{let (m,t,n)=T(o,d);if m==0{return v(0.7,0.6,1.)*P(1.-d.z,4.)}let h=o+d*t;let (l,r)=(u(v(9.+R(),9.+R(),16.)+h*-1.),d+n*i(n,d)*-2.);let mut b=i(l,n);if b<0.||T(h,l).n0()!=0{b=0.}let p=P(i(l,r)*(b>0.)as f,99.);if m==1{let h=h*0.2;(if(h.x.ceil()+h.y.ceil())as i&1==1{v(3.,1.,1.)}else{v(3.,3.,3.)})*(b*0.2+0.1)}else{v(p,p,p)+S(h,r)*0.5}}fn main(){print("P6 512 512 255 ");let g=u(v(-5.5,-16.,0.));let a=u(c(v(0.,0.,1.),g))*0.002;let b=u(c(g,a))*0.002;let c=(a+b)*-256.+g;for y in range(-511,1){for x in range(-511,1){let mut p=v(13.,13.,13.);for _ in range(0,64){let t=(a*(-0.5+R())+b*(-0.5+R()))*99.;p=S(v(17.,16.,8.)+t,u(t*-1.+(a*(-x as f+R())+b*(-y as f+R())+c)*16.))*3.5+p}std::io::stdout().write([p.x as u8,p.y as u8,p.z as u8])}}}