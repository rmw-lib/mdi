<h1 align="center">mdi v`> ./.version`</h1>
<p align="center">
<a href="#en">English</a>
|
<a href="#zh"> 中文说明 </a>
</p>

---

<span id="en"></span>

## English Readme

markdown include code / version / markdown ...

### Preface

Embedding version numbers and demo code in `readme.md` is a very common requirement.

There are some similar tools on the market, but none of them works well. So I wrote one myself ( executable size 200 KB ).

The function is to write a statement like `\> ./demo.js` in markdown to embed the code, and the screenshot of the generated content is as follows :

![](https://raw.githubusercontent.com/gcxfd/img/gh-pages/i9g9We.png)

In addition, with the [markdown translation tool](https://rmw.link/log/2021-12-09-markdown-translate) I wrote [based on deepl](https://rmw.link/log/2021-12-09-markdown-translate), you can automatically translate and render Chinese and English in the same `readme.md` file.

Then set an in-text anchor point (as below) at the top of the github readme, and click it to jump to the various language versions of the instructions, which will be a good user experience.

![](https://raw.githubusercontent.com/gcxfd/img/gh-pages/YQfKiS.png)

### Install

[Download from github](https://github.com/rmw-lib/mdi/releases) or `cargo install mdi`

### Use

> ./example.md

Will search `xxx.mdi.md` in the directory , replace `> ./xxx.rs` into embed code and output it to `xxx.md`.

If the embed is a markdown file, the references are rendered recursively.

See example [readme.mdi.md](https://raw.githubusercontent.com/rmw-lib/mdi/master/readme.mdi.md) , and the resulting file [readme.md](https://raw.githubusercontent.com/rmw-lib/mdi/master/readme.md)

Demo for include code :

  > ./demo.js

### About

This project is part of **[rmw.link](//rmw.link)** Code Project

![rmw.link logo](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)

---

<span id="zh"></span>

> ./zh.md
