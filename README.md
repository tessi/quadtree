# Quadtree

A quadtree is a tree data structure in which each internal node has exactly four children. Quadtrees are most often used to partition a two-dimensional space by recursively subdividing it into four quadrants or regions. The regions may be square or rectangular, or may have arbitrary shapes.

Here, I implement a quadtree (and some variations) in Rust just to play with them, benchmark and learn from it. Probably not the best production code :D

## Benchmarks

```bash
cargo bench
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
