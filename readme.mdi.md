# mdi

<a href="https://docs.rs/mdi"><img src="https://img.shields.io/badge/RUST-API%20DOC-blue?style=for-the-badge&logo=docs.rs&labelColor=333" alt="Api Doc"></a>
<a href="https://github.com/rmw-lib/mdi/releases"><img src="https://img.shields.io/badge/Download-EXE-090?style=for-the-badge&logo=rust&labelColor=333" alt="Download"></a>

[English](#english-readme) | [中文说明](#中文说明)

---

## English Readme

markdown include code / version / markdown ...

### Preface

Embedding version numbers and demo code in `readme.md` is a very common requirement.

There are some similar tools on the market, but none of them works well. So I wrote one myself ( executable size 200 KB ).

The function is to write a statement like `‍> ./demo.js` in markdown to embed the code, and the screenshot of the generated content is as follows :

![](https://raw.githubusercontent.com/gcxfd/img/gh-pages/i9g9We.png)

In addition, with the [markdown translation tool](https://rmw.link/log/2021-12-09-markdown-translate) based on deepl , you can automatically translate and render Chinese and English in the same `readme.md` file.

Then set an in-text anchor point (like this readme) at the top of the github readme, and click it to jump to the various language versions of the instructions, which will be a good user experience.

### Install

[Download from github](https://github.com/rmw-lib/mdi/releases) or `cargo install mdi`

### Use

> ./example.md

Will search `xxx.mdi.md` in the directory , replace `> ./xxx.rs` into embed code and output it to `xxx.md`.

If the embed is a markdown file, the references are rendered recursively.

See example [readme.mdi.md](https://raw.githubusercontent.com/rmw-lib/mdi/master/readme.mdi.md) , and the resulting file [readme.md](https://raw.githubusercontent.com/rmw-lib/mdi/master/readme.md)

Demo for include code :

  > ./demo.js

The `~` in `> ~/xxx.rs` indicates a file reference based on the project's root directory.

`mdi` will look up the `.git` directory from the current directory (or the command line argument `[dir]`), using the first directory where the `.git` folder exists as the root of the project, or the current directory if it is not found.

Paths ignored in `.gitignore` are ignored.

### About

This project is part of **[rmw.link](//rmw.link)** Code Project

![rmw.link logo](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)

---

> ./zh.md
