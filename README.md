# wasm-frontmatter

`wasm-frontmatter` extracts the frontmatter data from markdown.

[![Tests](https://github.com/joshfinnie/wasm-frontmatter/workflows/Tests/badge.svg)](https://github.com/joshfinnie/wasm-frontmatter/actions?query=workflow%3ATests) [![NPM](https://img.shields.io/npm/v/wasm-frontmatter)](https://www.npmjs.com/package/wasm-frontmatter)

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

## Options

### options.excerpt

**Type**: `Boolean`

**Default**: `false`

Extracts an excerpt that directly follows front-matter, or is the first thing in the string if no front-matter exists.

If set to `excerpt: true`, it will look for the frontmatter delimiter, `---` and grab everything leading up to it.

**Example**

```js
const str = '---\nfoo: bar\n---\nThis is an excerpt.\n---\nThis is content';
const file = matter(str, { excerpt: true });
```

Results in:

```js
{
  content: 'This is an excerpt.\n---\nThis is content',
  data: { foo: 'bar' },
  excerpt: 'This is an excerpt.\n'
}
```

### options.excerpt_separator

**Type**: `String`

**Default**: `---`

Allows for a custom separator to use for excerpts.

**Example**

```js
const str = '---\nfoo: bar\n---\nThis is an excerpt.\n<!-- end -->\nThis is content';
console.log(matter(str, {excerpt_separator: '<!-- end -->'}));
```

Results in:

```js
{
  content: 'This is an excerpt.\n<!-- end -->\nThis is content',
  data: { foo: 'bar' },
  excerpt: 'This is an excerpt.\n'
}
```

### options.delimiters

**Type**: `String`

**Default**: `---`

Allows for a custom delimiter to define the frontmatter.

**Example:**

```js
const str = '~~~\nfoo: bar\n~~~\nThis is content';
console.log(matter(str, {delimiter: '~~~'}));
```
Results in:

```js
{
  content: 'This is content',
  data: { foo: 'bar' },
  excerpt: ''
}
```

## Develop

Modify `src/lib.rs` and run `make build` to update the package. Can run tests using the included `index.js` file. Or write tests and run `make test`.

## Colophon

This package is built using [Rust](https://www.rust-lang.org/), [wasm_bindgen](https://github.com/rustwasm/wasm-bindgen), and [wasm-pack](https://github.com/rustwasm/wasm-pack). Hoping this finds itself to be fast since it's using wasm, but I don't know. This is very much experimental.
