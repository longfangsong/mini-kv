# Talent Plan Rust Project 1

This is my implement of PingCAP Talent Plan's Rust Project 1.

The major difference of my implementation is I wrote an an abstraction level 
over the key-value storage. So it is easy to use another storage backend, eg. `BTreeMap`
instead of `HashMap`.

And I also add several alias for sub-commands in `kvs.rs`.
