## 中文说明

markdown 中嵌入代码 / 版本号 / markdown ...

### 安装

[从 github 下载](https://github.com/rmw-lib/mdi/releases) 或者 `cargo install mdi`

### 使用

> ./example.md

会搜索目录下的 `xxx.mdi.md` 把 `> ./xxx.rs` 替换为代码内容嵌入 ，然后输出到 `xxx.md` 。

如果嵌入的是一个 markdown 文件，会递归渲染其中的引用。

查看演示文件 [readme.mdi.md](https://raw.githubusercontent.com/rmw-lib/mdi/master/readme.mdi.md)

嵌入代码演示 :

  > ./demo.js

### 关于

本项目隶属于 **人民网络 ([rmw.link](//rmw.link))** 代码计划。

![人民网络海报](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)
