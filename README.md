# Ray Tracing With Rust

## Introduction

参考：https://raytracing.github.io/

## Usage

```sh
# 默认的 master 分支只包含 InOneWeekend 的内容
make bt
make run

# 切到 the-next-week 分支
git checkout the-next-week
make bt
make run
```

## Todo

- [ ] 在数据结构中引入 Cow<'a, B> 结构避免不必要拷贝
- [ ] 尝试使用枚举优化 dyn trait，避免动态分发
