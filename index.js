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
