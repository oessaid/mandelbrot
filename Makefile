.PHONY=demo demo_time test doc

demo:
	cargo build --release && target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20 fast && feh mandel.png
demo_time:
	cargo build --release && time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20 fast && feh mandel.png
test:
	cargo test
doc:
	cargo doc --open
