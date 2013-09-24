![Raytraced Rust](https://raw.github.com/huonw/card-trace/master/rust.png)

A Rust version of the C++ "business card" ray tracer.

- original: <http://www.cs.utah.edu/~aek/code/card.cpp>
- explanation: <http://fabiensanglard.net/rayTracing_back_of_business_card/index.php>
- "clean" C++ code: <https://gist.github.com/kid0m4n/6680629> (the
  `original.cpp` in this repo, edited to render `Rust` rather than
  `Go`.)
- Go version: <https://github.com/kid0m4n/gorays>
- Prompted by this ML post: <https://mail.mozilla.org/pipermail/rust-dev/2013-September/005735.html>

## Run it

- `rustc --opt-level=3 bin.rs`
- `./bin > rust.ppm`
- `convert rust.{ppm,png}`
- Open `rust.png` with your favourite viewer

## Compare it

~~~
$ gcc -O3 -lm original.cpp -o cxx
$ time ./cxx > rust.ppm

real    0m17.923s
user    0m17.900s
sys     0m0.012s

$ clang -O3 -lm original.cpp -o cxx
$ time ./cxx > rust.ppm

real    0m13.755s
user    0m13.744s
sys     0m0.000s

$ rustc --opt-level=3 bin.rs -o rust
$ time ./rust > rust.ppm

real    0m17.883s
user    0m17.368s
sys     0m0.500s
~~~

These all render `Rust`.
