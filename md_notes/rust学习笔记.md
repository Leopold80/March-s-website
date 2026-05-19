---
title: Rust 学习笔记
date: 2024-01-20
---

最近整理了一些 Rust 学习心得。

## 所有权（Ownership）

Rust 最特别的概念就是所有权系统。

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 的所有权转移给 s2
    // println!("{}", s1);  // 这会编译错误！
    println!("{}", s2);  // 正确
}
```

## 借用（Borrowing）

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

使用 `&` 借用而不是转移所有权。

## 生命周期（Lifetimes）

生命周期标注帮助编译器理解引用的有效范围。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

后续会继续更新学习笔记 📚
