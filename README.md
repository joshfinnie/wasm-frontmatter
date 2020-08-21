# wasm-frontmatter

`wasm-frontmatter` extracts the frontmatter data from markdown.

## Install

```
$ npm install --save wasm-frontmatter
```

## Use

```
const {matter} = require('wasm-frontmatter');

const long_markdown = `---
title: "long form content"
description: "Front matter"
categories:
  - "test"
  - "categories"
---

This is content`;

results = matter(long_markdown);

console.log(results.data.title);
```

## Develop

Modify `src/lib.rs` and run `make build` to update the package. Can run tests using the included `index.js` file. Or write tests and run `make test`.

## Colophon

This package is built using [Rust](https://www.rust-lang.org/), [wasm_bindgen](https://github.com/rustwasm/wasm-bindgen), and [wasm-pack](https://github.com/rustwasm/wasm-pack). Hoping this finds itself to be fast since it's using wasm, but I don't know. This is very much experimental.
