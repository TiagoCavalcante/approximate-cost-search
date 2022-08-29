# approximate-cost-search

Find a path with a (bad) approximated cost as fast as a Dijkstra

## How to run?

```sh
$ cargo run --release
```

## How fast is it?

Here is the output of the benchmark of the algorithm for a graph with 20 thousand vertices and density of 0.01:
```
Fill the graph - 3.07s
Approximate cost search - 33.26ms
The path is valid
```

Yep, that is milliseconds, not seconds.

## Should I use it?

No, you shouldn't use this for any real world case, this is only a proof of concept that I created this because I wanted to adapt [approximate-length-search](https://github.com/TiagoCavalcante/approximate-length-search). This is almost instantaneous but almost never gives a good approximation.

If you want an algorithm for finding paths with a specific length you should see [fixed-length-search](https://github.com/TiagoCavalcante/fixed-length-search) and [fixed-cost-search](https://github.com/TiagoCavalcante/fixed-cost-search), both are also blazingly fast.
