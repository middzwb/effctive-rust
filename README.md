# effctive-rust

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