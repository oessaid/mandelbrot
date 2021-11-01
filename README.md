# Mandelbrot plotter

## Running

```sh
make demo
```

## Performance

### Naive chunk splitting (bands)

Single-threaded version (`time` output):

```text
3.78 user
0.00 system
0:03.78 elapsed
100% CPU (0avgtext+0avgdata 16708maxresident)k
0inputs+3656outputs (0major+3641minor)pagefaults 0swaps
```

Multi-threaded version (using `crossbeam`):

```text
3.89 user
0.00 system
0:01.11 elapsed
350% CPU (0avgtext+0avgdata 16932maxresident)k
0inputs+3656outputs (0major+3669minor)pagefaults 0swaps
```

Multi-threaded version (using `rayon`):

```text
3.80 user
0.01 system
0:01.12 elapsed
339% CPU (0avgtext+0avgdata 15528maxresident)k
0inputs+3656outputs (0major+3693minor)pagefaults 0swaps
```

Multi-threaded version (using `rayon`, one task per pixel row):

```text
4.17 user
0.03 system
0:00.42 elapsed
981% CPU (0avgtext+0avgdata 15328maxresident)k
0inputs+3656outputs (0major+3737minor)pagefaults 0swaps
```

## Example

![Mandelbrot](mandel.png?raw=true "Mandelbrot")
