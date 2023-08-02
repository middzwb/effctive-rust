# effctive-rust

## cargo

包管理工具。

cargo new
cargo build
cargo run

配置下载镜像为国内镜像。

## 基本类型

* 避免在浮点数上测试相等性
* 当结果在数学上可能存在未定义时，需要格外的小心

字符类型：unicode，所以字符占 4 字节。

单元类型 `()`。

## 所有权

## 复合类型

### 切片 slice

## 坑

1. win7 下安装 rust

之前已经安装过 tdm-gcc， rust-init 安装的时候很顺利；cargo build 时有报错:
```
error: linking with `x86_64-w64-mingw32-gcc` failed: exit code: 1
  |--remove-destination
  
  ......
  
  = note: D:/software/tdmgcc/bin/../lib/gcc/x86_64-w64-mingw32/10.3.0/../../../../x86_64-w64-mingw32/bin/ld.exe: cannot find -lgcc_eh
          collect2.exe: error: ld returned 1 exit status
```

google 后，将 libgcc.a 放到 lib 目录下， 并重命名为 libgcc_eh.a ， 解决。
[not find -lgcc_eh](https://blog.csdn.net/zhi443/article/details/54094325)