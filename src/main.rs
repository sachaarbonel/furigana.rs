use nom::{
    bytes::complete::{tag, take_while1},
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Element {
    Ruby(HashMap<String, Element>), //<ruby>RubyText</ruby>String
    RubyText(String),               //<rt>BaseText
}

fn ruby(i: &str) -> IResult<&str, Element> {
    let (i, (_, rt, _, base_text)) = tuple((tag("<ruby>"), rt, tag("</ruby>"), alphanumeric))(i)?;
    let mut dic = HashMap::new();
    dic.insert(String::from(base_text), rt);
    Ok((i, Element::Ruby(dic)))
}

fn rt(i: &str) -> IResult<&str, Element> {
    let (i, (_, base_text)) = tuple((tag("<rt>"), alphanumeric))(i)?;
    Ok((i, Element::RubyText(String::from(base_text))))
}

fn alphanumeric(i: &str) -> IResult<&str, &str> {
    take_while1(is_japanese)(i)
}

fn is_japanese(data: char) -> bool {
    match data {
        '\u{3000}'..='\u{303F}' => true,
        '\u{4E00}'..='\u{9FCB}' => true,
        '\u{F900}'..='\u{FAFA}' => true,
        '\u{3400}'..='\u{4DB5}' => true,
        '\u{2E80}'..='\u{2FD5}' => true,
        '\u{3041}'..='\u{3096}' => true,
        '\u{30A0}'..='\u{30FF}' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_japanese_test() {
        let mut chars = "同".chars();
        assert_eq!(is_japanese(chars.next().unwrap()), true);
    }

    #[test]
    fn rt_test() {
        //https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-ruby-element
        let text = "<rt>どう";
        assert_eq!(rt(text), Ok(("", Element::RubyText("どう".to_string()))))
    }
}

fn main() {
    println!("Hello, world!");
}
