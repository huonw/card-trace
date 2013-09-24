#include <stdlib.h>
#include <stdio.h>
#include <math.h>

typedef int i;       //Save space by using 'i' instead of 'int'
typedef float f;     //Save even more space by using 'f' instead of 'float'

//Define a vector class with constructor and operator: 'v'
struct v {
  f x,y,z;  // Vector has three float attributes.
  v operator+(v r){return v(x+r.x,y+r.y,z+r.z);} //Vector add
  v operator*(f r){return v(x*r,y*r,z*r);}       //Vector scaling
  f operator%(v r){return x*r.x+y*r.y+z*r.z;}    //Vector dot product
  v(){}                                  //Empty constructor
  v operator^(v r){return v(y*r.z-z*r.y,z*r.x-x*r.z,x*r.y-y*r.x);} //Cross-product
  v(f a,f b,f c){x=a;y=b;z=c;}            //Constructor
  v operator!(){return *this*(1 /sqrt(*this%*this));} // Used later for normalizing the vector
};

// NOTE, edited to display `Rust`
i G[]={301252, 336932, 402628, 468255, 304324, 458756, 0, 0, 0};

// Random generator, return a float within range [0-1]
f R(){return (f)rand()/RAND_MAX;}

//The intersection test for line [o,v].
// Return 2 if a hit was found (and also return distance t and bouncing ray n).
// Return 0 if no hit was found but ray goes upward
// Return 1 if no hit was found but ray goes downward
i T(v o,v d,f& t,v& n) {
  t=1e9;
  i m=0;
  f p=-o.z/d.z;

  if(.01<p)
    t=p,n=v(0,0,1),m=1;

  //The world is encoded in G, with 9 lines and 19 columns
  for(i k=19;k--;)  //For each columns of objects
  for(i j=9;j--;)   //For each line on that columns

  if(G[j]&1<<k) { //For this line j, is there a sphere at column i ?
    // There is a sphere but does the ray hits it ?
    v p=o+v(-k,3,-j-4);
    f b=p%d,c=p%p-1,q=b*b-c;

    // Does the ray hit the sphere ?
    if(q>0) {
      //It does, compute the distance camera-sphere
      f s=-b-sqrt(q);

      if(s<t && s>.01)
      // So far this is the minimum distance, save it. And also
      // compute the bouncing ray vector into 'n'
      t=s, n=!(p+d*t), m=2;
    }
  }

  return m;
}

// (S)ample the world and return the pixel color for
// a ray passing by point o (Origin) and d (Direction)
v S(v o,v d) {
  f t;
  v n;

  //Search for an intersection ray Vs World.
  i m=T(o,d,t,n);

  if(!m) // m==0
    //No sphere found and the ray goes upward: Generate a sky color
    return v(.7,.6,1)*pow(1-d.z,4);

  //A sphere was maybe hit.

  v h=o+d*t,                    // h = intersection coordinate
  l=!(v(9+R(),9+R(),16)+h*-1),  // 'l' = direction to light (with random delta for soft-shadows).
  r=d+n*(n%d*-2);               // r = The half-vector

  //Calculated the lambertian factor
  f b=l%n;

  //Calculate illumination factor (lambertian coefficient > 0 or in shadow)?
  if(b<0||T(h,l,t,n))
    b=0;

  // Calculate the color 'p' with diffuse and specular component
  f p=pow(l%r*(b>0),99);

  if(m&1) {   //m == 1
    h=h*.2; //No sphere was hit and the ray was going downward: Generate a floor color
    return((i)(ceil(h.x)+ceil(h.y))&1?v(3,1,1):v(3,3,3))*(b*.2+.1);
  }

  //m == 2 A sphere was hit. Cast an ray bouncing from the sphere surface.
  return v(p,p,p)+S(h,r)*.5; //Attenuate color by 50% since it is bouncing (* .5)
}

// The main function. It generates a PPM image to stdout.
// Usage of the program is hence: ./card > erk.ppm
i main() {
  printf("P6 512 512 255 "); // The PPM Header is issued

  // The '!' are for normalizing each vectors with ! operator.
  v g=!v(-5.5,-16,0),       // Camera direction
    a=!(v(0,0,1)^g)*.002, // Camera up vector...Seem Z is pointing up :/ WTF !
    b=!(g^a)*.002,        // The right vector, obtained via traditional cross-product
    c=(a+b)*-256+g;       // WTF ? See https://news.ycombinator.com/item?id=6425965 for more.

  for(i y=512;y--;)   //For each column
  for(i x=512;x--;) {   //For each pixel in a line
    //Reuse the vector class to store not XYZ but a RGB pixel color
    v p(13,13,13);     // Default pixel color is almost pitch black

    //Cast 64 rays per pixel (For blur (stochastic sampling) and soft-shadows.
    for(i r=64;r--;) {
      // The delta to apply to the origin of the view (For Depth of View blur).
      v t=a*(R()-.5)*99+b*(R()-.5)*99; // A little bit of delta up/down and left/right

      // Set the camera focal point v(17,16,8) and Cast the ray
      // Accumulate the color returned in the p variable
      p=S(v(17,16,8)+t, //Ray Origin
      !(t*-1+(a*(R()+x)+b*(y+R())+c)*16) // Ray Direction with random deltas
                                         // for stochastic sampling
      )*3.5+p; // +p for color accumulation
    }

    printf("%c%c%c",(i)p.x,(i)p.y,(i)p.z);
  }
}
