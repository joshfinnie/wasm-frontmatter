const {matter} = require('./pkg/');


console.log("################ TEST 0 ################");
console.log(`{
  content: 'This is an excerpt.\n---\nThis is content',
  data: { foo: 'bar' },
  excerpt: 'This is an excerpt.\n'
}`);
console.log(matter(`---
foo: bar
---
This is an excerpt.
---
This is content`));

console.log("################ TEST 1 ################");
console.log(`{
  content: 'This is content.',
  data: { title: 'Front Matter' },
  excerpt: ''
}`);
console.log(matter('---\ntitle: Front Matter\n---\nThis is content.'));
console.log("\n");

console.log("################ TEST 2 ################");
console.log(`{ content: 'Other stuff', data: { title: 'Home' }, excerpt: '' }`);
console.log(matter('---\ntitle: Home\n---\nOther stuff'));
console.log("\n");

const long_markdown = `---
title: "long form content"
description: "Front matter"
categories:
  - "test"
  - "categories"
---

Let's add an excerpt!

---

This is content`;

console.log("################ TEST 3 ################");
console.log(`{
  content: '\nLet's add an excerpt!\n\n---\n\nThis is content',
  data: {
    title: 'long form content',
    description: 'Front matter',
    categories: [ 'test', 'categories' ]
  }
  excerpt: '\nLet's add an excerpt!\n\n'
}`);
console.log(matter(long_markdown));
console.log("\n");

data = matter(long_markdown);
console.log(data.data.title)
console.log("\n");

console.log("################ TEST 4 ################");
console.log(`{
  content: 'this is an excerpt!\n\n---\n\nthis is content!',
  data: {}
  excerpt: 'this is an excerpt!\n\n'
}`);
console.log(matter("this is an excerpt!\n---\nthis is a content!"))

console.log("################ TEST 5 ################");
opt = {excerpt: true};

console.log(matter(long_markdown, opt));
console.log("should have excerpt!\n");

console.log(matter(long_markdown));
console.log("should not have excerpt!\n");

console.log("################ TEST 6 ################");
const long_markdown_different_separator = `---
title: "long form content"
description: "Front matter"
categories:
  - "test"
  - "categories"
---

Let's add an excerpt!

<!-- end -->

This is content`;

opt = {excerpt_separator: "<!-- end -->"};

console.log(matter(long_markdown_different_separator, opt));
console.log("should have excerpt!\n");

console.log(matter(long_markdown_different_separator, {excerpt: true, excerpt_separator: "<!-- end -->"}));
console.log("should have excerpt!\n");

console.log("################ TEST 7 ################");
const fun_markdown = `~~~
title: "fun markdown"
~~~
Content`;

console.log(matter(fun_markdown, {delimiters: "~~~"}));
console.log("should not have excerpt!\n");
