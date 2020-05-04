# Talent Plan Rust Project 2

This is my implement of PingCAP Talent Plan's Rust Project 2.
The major difference of this and the example project is that I used a simplified version of lsm-tree as storage.

## simplified version of lsm-tree

There're only two levels.
One is the unordered, read/append-only, may contains put/delete operation with duplicated keys, small tree
the other is the ordered, read-only, unique key-value only, large tree.

When the small tree's size reach some certain limit and compaction triggered,
merge-sort would be used to compact the small tree into the large one.

Since the length of the input is not a fixed value, a "meta" file is need to
know the offset of each k-v pair in large tree, this approach looks a little like WiscKey

Maybe I'll replace this with a real lsm-tree in the future.

# Rust 编程原理与实践 项目 2

此为我在Talent Plan中Rust编程原理与实践中项目1的实现。

与样例实现的主要差别在于我实现了一个简化版的LSM-Tree作为底层存储。

## 简化版LSM-Tree

只分为两层，一层是无序的，只允许读/顺序写的，保存了最近的写操作的，可能有重复key的小"树"。

第二层是有序的，只读的，只保存唯一键值对的大"树"。

当小树达到一定大小时，将会进行compact操作，将小树和原大树归并入新的大树。

由于使用的键和值大小均不定，为了在大"树"中进行二分查找时随机访问方便，
将键对应的在大"树"中的offset存入一个meta文件，这种方式有点类似WiscKey的思想。

将来考虑使用真正的lsm-tree代替这一简化版。