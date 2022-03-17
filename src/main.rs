use std::str::Chars;
use std::iter::Peekable;

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
    fn consume_while<F>(&mut self, condition: F) -> String
    //复杂trait约束的写法
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| {print!("{:?}",c);condition(*c)}) {
            print!("{}",11);
            result.push(self.chars.next().unwrap())
        }
        result
    }
}
fn main(){
    let mut a = HtmlParser::new("981b ou18g  o813grg ");
    print!("{:?}",a.chars.peek());
    print!("{:?}",a.chars.peek());
    print!("{:?}",a.chars.next());
    print!("{:?}",a.chars.peek());
    print!("{:?}",a.chars.peek());
    let result = &a.consume_while(char::is_whitespace);
    print!("{:?}",result)
}