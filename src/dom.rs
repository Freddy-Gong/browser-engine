use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(PartialEq, Eq)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}
#[derive(PartialEq, Eq, Clone)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}
#[derive(PartialEq, Eq, Clone)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

impl ElementData {
    fn new(tag_name: String, attributes: AttrMap) -> ElementData {
        ElementData {
            tag_name,
            attributes,
        }
    }
    //获取元素的id
    fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }
    //获取元素的class
    //因为class是可以有多个的 所以我们要把它用空格切分开
    //放在一个HashSet中
    fn get_Classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(s) => s.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}
type AttrMap = HashMap<String, String>;

impl Node {
    fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            children,
            node_type,
        }
    }
}
//为Node这个结构实现fmt::Deubg这个trait 这样可以使他被打印在标准输出中
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //这里刚写的时候会飘红，因为node_type的类型NodeType也是不支持
        //fmt::Debug这个trait的
        //所以下面还要为NodeType实现这个trait
        write!(f, "{:?}", self.node_type)
    }
}
//所以下面还要为NodeType实现这个trait
impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            //这里因为有 | 运算符，所以要在枚举变量前加一个ref，才能使用
            NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
            //这里不加ref *self会爆红 但是把*去掉就行了
            NodeType::Element(ref e) => write!(f, "{:?}", e),
        }
    }
}
//但是ElementData没有实现Copy这个trait 所以为她实现一下
impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attributes_string = String::new();
        for (attr, value) in self.attributes.iter() {
            attributes_string.push_str(&format!("{}=\"{}\"", attr, value));
        }
        write!(f, "<{},{}>", self.tag_name, attributes_string)
    }
}
//为了打印的漂亮一些，会在每一行添加缩进
fn pretty_print(n: &Node, indent_size: usize) {
    //构建缩进
    let indent = (0..indent_size).map(|_| " ").collect::<String>();
    //这两种写法，不是借node_type就是接t e c这三个 都是可以的
    //有什么区别么？
    match &n.node_type {
        NodeType::Text(t) => println!("{}{}", indent, t),
        NodeType::Element(e) => println!("{}{:?}", indent, e),
        NodeType::Comment(c) => println!("{}<!---{}--->", indent, c),
    }
    // match n.node_type {
    //     NodeType::Text(ref t) => println!("{}{}", indent, t),
    //     NodeType::Element(ref e) => println!("{}{:?}", indent, e),
    //     NodeType::Comment(ref c) => println!("{}<!---{}--->", indent, c),
    // }

    for child in n.children.iter() {
        //递归
        pretty_print(&child, indent_size + 2)
    }

    match &n.node_type {
        NodeType::Element(e) => println!("{}<{}/>", indent, e.tag_name),
        _ => {}
    }
}
