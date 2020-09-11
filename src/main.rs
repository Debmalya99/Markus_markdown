extern crate markdown;
use std::fs;

fn main() {
    let file_contents:String = fs::read_to_string("/home/dibakar/Debmalya-workspace/hobby/code-projects/markus/test.md").unwrap();
    // println!("{}",file_contents);
    let converted_html:String = markdown::to_html(&file_contents);
    // println!("{}",converted_html);
    let mut final_string = String::from("<script src=\"https://polyfill.io/v3/polyfill.min.js?features=es6\"></script>\n
<script id=\"MathJax-script\" async src=\"https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js\"></script>\n\n");

    final_string.push_str(&converted_html);
    println!("{}",final_string);
}
