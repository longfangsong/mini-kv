# Talent Plan Rust Project 3

This is my implement of PingCAP Talent Plan's Rust Project 3.
The major difference of this and the example project is
- I rewrite the benchmark code according to my own understanding of the requirements.
- Use `bincode` instead of `serde-json` as Serializer

# Rust 编程原理与实践 项目 3

此为我在Talent Plan中Rust编程原理与实践中项目3的实现。

与样例实现的主要差别在于
- 我根据我的理解重写了benchmark代码。给的参考实现和要求似乎对不起来。
- 将`bincode`而非`serde-json`用于serde的序列化和反序列化。（个人认为CS架构下使用二进制的数据交换格式是一个较好的选择）
