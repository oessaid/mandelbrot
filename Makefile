.PHONY=demo demo_time

demo:
	cargo build --release && target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20 && feh mandel.png
demo_time:
	cargo build --release && time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20 && feh mandel.png
