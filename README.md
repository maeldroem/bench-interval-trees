# `bench-interval-trees`

This repository is a benchmarking experiment in order to compare the performance of different
data structures to store intervals.

# Tested data structures

The following data structures will be benchmarked

- [Segment Tree](https://en.wikipedia.org/wiki/Segment_tree)
- Triple* [AVL Tree](https://en.wikipedia.org/wiki/AVL_tree)

*: One sorted by `(from, -to)`, one sorted by `(to, -from)`,
and one storing misc data (still undecided at this time, but perhaps distances between different intervals)

# Tested operations

## Structural operations

- Build from an unsorted vector of intervals
- Remove a lot of intervals from the structure
- Insert a lot of intervals in the structure
- Update a lot of intervals in the structure
- Alternate randomly between removing and inserting a lot of intervals

## Boolean operations

- Overlap check - does a given interval overlap any in the tree?

## Range operations

- Find the earliest (and longest in case of equality) interval
- Find the latest (and longest in case of equality) interval
- Flatten all intervals into a vector of non-overlapping intervals
- Find the longest interval which midpoint is closest to the midpoint of the earliest start and latest end
- Find all overlapping intervals of a given interval
- Find closest interval beginning after a given number
- Remove multiple entire sections (represented as intervals) off the structure
- Retrieve hierarchy of a given interval - The returned value must be a tree where leaves are entirely contained within the given interval. The parent of those leaves must entirely contain its children, and this rule must be followed in the entire tree. Ancestors of the given interval must also be returned.

## Set operations

- Merge two instances of the structure (Union)
- Intersect two instances of the structure
- Difference between instance A and instance B

# Hypothesis

I believe that using a triple AVL tree will lead to more flexibility and increased performance
when it comes to boolean operations, range operations, and set operations as well as operations
such as finding neighboring intervals.

There's nothing backing my hypothesis, this is only based on gut feeling, but it's the principle
of a hypothesis! I will happily accept the outcome of this experiment, whether it contradicts
or confirms my hypothesis.

This gut feeling is based on my doubts about the performance of segment trees when it comes to updates.
Although I believe it will be highly performant for queries like retrieving the hierarchy of an interval
or finding overlapping intervals.

# Results

## Run 1 - 2025-03-XX

### Hardware

| Part | Description |
| ----- | ----- |
| CPU | 24 × AMD Ryzen 9 7900X3D 12-Core Processor |
| RAM | 32Gb |
| OS version / Kernel | 6.13.5-2-MANJARO (64-bit) |

### Setup

| Object | Description |
| ----- | ----- |
| Project version | `0.1.0` |
| `rustc` version | `1.85.0` |
| `criterion` version | `0.5.1` |

### Results

Not available

### Conclusion

Not available
