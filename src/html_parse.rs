use dom::{AttrMap, ElementData, Node, NodeType};
//Peekable文档https://doc.rust-lang.org/stable/std/iter/struct.Peekable.html
//运行peek()会返回下一个元素的可变引用需要包裹一个迭代器
use std::iter::Peekable;
//Chars是对str的一个迭代器 有需要生命周期
use std::str::Chars;

pub struct HtmlParser<'a> {
    chars: Peekable<Chars<'a>>, //储存html字符串的,一个字符一个字符的运行
    node_q: Vec<String>,        //储存node的tag_name的
}

impl<'a> HtmlParser<'a> {
    pub fn new(full_html: &str) -> HtmlParser {
        HtmlParser {
            chars: full_html.chars().peekable(),
            node_q: Vec::new(),
        }
    }
    //把html Element 转化为node
    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        //调用peek()会返回下一个字符的引用
        while self.chars.peek().is_some() {
            //判断是不是空格,如果是空格就后移，不是就停住,把空格组成的字符串返回
            self.consume_while(char::is_whiterspace);
            //判断标签
            if self.chars.peek().map_or(false, |c| *c == '<') {
                slef.chars.next();//next会将字符位置后移，peek则不会
                //判断闭合标签
                if slef.chars.peek().map_or(false, |c| *c == '/') {
                    slef.chars.next();
                    //去除空格
                    self.consume_while(char::is_whiterspace);
                    //获取标签名称
                    let close_tag_name = self.consume_while(is_valid_tag_name);
                    //标签闭合
                    self.consume_while(|x| x != '>');
                    self.chars.next();
                    self.node_q.push(close_tag_name);
                    break;
                } else if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
                    nodes.push(self.parse_comment_node())
                //判断标签起始
                }else{
                    //解析标签的内容
                    let mut node = self.parse_node();
                    let insert_index = nodes.len();

                    match &node.node_type {
                        NodeType::Element(e) => if self.node_q.len() > 0 {
                            let assumed_tag = self.node_q.remove(0);
                            //如果当前tagname和第一个tagnema不想等，则插入最后
                            if e.tag_name != assumed_tag {
                                nodes.append(&mut node.children);
                                self.node_q.insert(0,assumed_tag)
                            }
                            //那相等呢？？把一个移除了？？什么鬼？
                        },
                        _ => {}
                    }

                    nodes.insert(insert_index, node)
                }
            //不是标签就parse文字
            }else {
                nodes.push(self.parse_text_node())
            }
        }
        nodes
    }

    fn parse_node(&mut self)->Node{
        let tagname = self.consume_while(is_valid_tag_name);
        let attributes = self.parse_attributes();

        let elem = ElementData::new(tagname,attributes);
        let chidlren = self.parse_nodes()
        Node::new(NodeType::Element(elem),children)
    }

    fn parse_text_node(&mut self) -> Node {
        let mut text_content = String::new();

        while self.chars.peek().map_or(false,|c| *c != '<'){
            let whitespace = self.consume_while(char::is_whitespace);
            if whitespace.len() > 0 {
                text_content.push(' ');
            }
            let text_patr = self.consume_while(|x| !x.is_whitespace() && x != '<');
            text_content.push_str(&text_part);
        }
        Node::new(NodeType::Text(text_content),Vec::new())
    }

    fn parse_comment_node(&mut self) -> Node {
        let comment_content = String::new()
        //<!---  ----->判断这种注释符号的
        if self.chars.peek().map_or(false,|c| *c == '-'){
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '-'){
                self.chars.next()
            }else{
                self.consume_while(|c| c!= '>');
                return Node::new(NodeType::Comment(comment_content),Vec::new())
            }
        }else{
            self.consume_while(|c| c != '>');
            return Node::new(NodeType::Comment(comment_content),Vec::new())
        }

        if self.chars.peek().map_or(false,|c| *c == '>'){
            self.chars.next()
            return Node::new(NodeType::Comment(comment_content),Vec::new())
        }

        if self.chars.peek().map_or(false, |c| *c == '-'){
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '>'){
                self.chars.next();
                return Node::new(NodeType::Comment(comment_content),Vec::new());
            }else{
                comment_content.push('-');
            }
        }

        while self.chars.peek().is_some() {
            comment_content.push_str(&self.consume_while(|c| c!= '<' && c != '-'));
            if self.chars.peek().map_or(false,|c| *c == '<'){
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '!'){
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '-'){
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-'){
                            self.consume_while(|c| c != '>');

                            return Node::new(NodeType::Comment(String::from("")),Vec::new())
                        }else {
                            comment_content.push_str("<!-")
                        }
                    }else if self.chars.peek().map_or(false,|c| *c == ' '){
                        self.chars.next()
                        if self.chars.peek().map_or(false,|c| *c == '-'){
                            self.chars.next()
                            if self.chars.peek().map_or(false,|c| *c == '-'){
                                self.chars.next()
                                if self.chars.peek().map_or(false,|c| *c == '-'){
                                    self.chars.next()
                                    if self.chars.peek().map_or(false,|c| *c == '>'){
                                        self.chars.next()
                                        return Node::new(
                                            NodeType::Comment(String::from("")),
                                            Vec::new(),
                                        )
                                    }else {
                                        comment_content.push_str("<! --")
                                    }
                                }else{
                                    comment_content.push_str("<! -")
                                }
                            }else{
                                comment_content.push_str("<! ")
                            }
                        }
                    }else{
                        comment_content.push_str("<!")
                    }
                }else{
                    comment_content.push_str("<")
                }
            }else if self.chars.peek().map_or(false,|c| *c == '-'){
                self.chars.next()
            }
        }
        Node::new(NodeType::Comment(comment_content),Vec::new())
    }

    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = AttrMap::new()

        while self.chars.peek().map_or(false, |c| *c != '>'){
            self.consume_while(char::is_whitespace);
            //属性名 
            let name = self.consume_while(|c| is_valid_attr_name(c)).to_lowercase();
            self.consume_while(char::is_whitespace);
            //属性值
            let value = if self.chars.peek().map_or(false,|c| *c == '='){
                self.chars.next()
                self.consume_while(char::is_whitespace);
                let s = self.parse_attr_value();
                self.consume_while(|c| !c.is_whitespace() && c != '>')
                self.consume_while(char::is_whitespace);
                s
            }else {
                "".to_string()
            };
            attributes.insert(name,value);
        }
        self.chars.next();

        attributes
    }

    fn parse_attr_value(&mut self) -> String {
        self.consume_while(char::is_whitespace);

        let result = match self.chars.peek() {
            Some(&c) if c == '"' || c == '\'' => {
                self.chars.next();
                let ret = self.consume_while(|x| x != c);
                self.chars.next();
                ret
            }
            _=>self.consume_while(is_valid_attr_value),
        };

        result
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    //复杂trait约束的写法
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap())
        }
        result
    }
}

fn is_valid_tag_name(ch: char) -> bool {
    //0~9就是数字
    //10-35就是字母
    ch.is_digit(36)
}

fn is_valid_attr_name(c: char) -> bool {
    !is_excluded_name(c) && !is_control(c)
}

fn is_control(ch: char) -> bool {
    match ch {
        '\u{007F}' => true,
        c if c >= '\u{0000}' && c <= '\u{001F}' => true,
        c if c >= '\u{0080}' && c <= '\u{009F}' => true,
        _ => false,
    }
}

fn is_excluded_name(c:char)->bool{
    match c {
        ' ' | '"' | '\'' | '>' | '/' | '=' =>true,
        _ => false
    }
}
