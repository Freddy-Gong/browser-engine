use dom::{AttrMap, ElementData, Node, NodeType};
//Peekable文档https://doc.rust-lang.org/stable/std/iter/struct.Peekable.html
//运行peek()会返回下一个元素的可变引用需要包裹一个迭代器
use std::iter::Peekable;
//Chars是对str的一个迭代器 有需要生命周期
use std::str::Chars;

pub struct HtmlParser<'a> {
    chars: Peekable<Chars<'a>>,
    node_q: Vec<String>,
}
