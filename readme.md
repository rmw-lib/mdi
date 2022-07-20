<!-- EDIT /Users/z/rmw/mdi/readme.md -->

<h1 align="center">mdi v0.0.23</h1>
<p align="center">
<a href="#en">English</a>
|
<a href="#zh"> 中文说明 </a>
</p>

---

<span id="en"></span>

## English Readme

markdown include code / version / markdown ...

### Install

[Download from github](https://github.com/rmw-lib/mdi/releases) or `cargo install mdi`

### Use

<!-- EDIT /Users/z/rmw/mdi/example.md -->

`mdi [dir]`

Will search `xxx.mdi.md` in the directory , replace `> ./xxx.rs` into embed code and output it to `xxx.md`.

If the embed is a markdown file, the references are rendered recursively.

See example [readme.mdi.md](https://raw.githubusercontent.com/rmw-lib/mdi/master/readme.mdi.md)

Demo for include code :

  [→ demo.js](./demo.js)

  ```js
  var a = 1;
  console.log(a);
  ```


### About

This project is part of **[rmw.link](//rmw.link)** Code Project

![rmw.link logo](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)

---

<span id="zh"></span>

<!-- EDIT /Users/z/rmw/mdi/zh.md -->

## 中文说明

markdown 中嵌入代码 / 版本号 / markdown ...

### 安装

[从 github 下载](https://github.com/rmw-lib/mdi/releases) 或者 `cargo install mdi`

### 使用

<!-- EDIT /Users/z/rmw/mdi/example.md -->

`mdi [dir]`

会搜索目录下的 `xxx.mdi.md` 把 `> ./xxx.rs` 替换为代码内容嵌入 ，然后输出到 `xxx.md` 。

如果嵌入的是一个 markdown 文件，会递归渲染其中的引用。

查看演示文件 [readme.mdi.md](https://raw.githubusercontent.com/rmw-lib/mdi/master/readme.mdi.md)

嵌入代码演示 :

  [→ demo.js](./demo.js)

  ```js
  var a = 1;
  console.log(a);
  ```


### 关于

本项目隶属于 **人民网络 ([rmw.link](//rmw.link))** 代码计划。

![人民网络海报](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)
