# Markus : simple Markdown to HTML converter

**Features**

1. Fast in performance thanks to the power of [Rust-lang](https://www.rust-lang.org/)
2. Supports equations written in [MathJax](https://www.mathjax.org/)
  * Supports math equations in a line/block. Does not support inline equations yet

**Future updates will include**

1. Adding custom stylesheets for customising the look of the document
2. Premade style templates for quickly producing good looking documents
3. Table of contents

**My primary intent in making this tool:** I take a lot of study notes. The computer is a great of way of taking notes. But, inserting equations, links, images are very inconvinient in traditional word processing software. So I made this tool

**Sample usage:**
>*markus input.md output.html*

**Example of Equation:**
Here is a nice litte equation(Faraday's Law) : $$\nabla \times E = -{\partial B \over \partial t}$$

This above equation might not render in this readme. But once you download and build this software, try converting this README.md to html and open the HTML in a browser, you will see the equation rendered in the resulting HTML.
