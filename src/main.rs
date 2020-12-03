use nom::{
    bytes::complete::{tag, take_while1},
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(PartialEq, Clone)]
enum RubyElement {
    Ruby(HashMap<String, RubyElement>), //<ruby>RubyText</ruby>baseText
    RubyText(String, String),           //kanji<rt>annotation
}

impl RubyElement {
    fn new(rt: RubyElement, base_text: String) -> Self {
        let dic: HashMap<String, RubyElement> = [(base_text, rt)].iter().cloned().collect();

        RubyElement::Ruby(dic)
    }
}

impl std::fmt::Display for RubyElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RubyElement::RubyText(kanji, annotation) => write!(f, "{}<rt>{}", kanji, annotation),
            RubyElement::Ruby(map) => {
                let entry = map.iter().next().unwrap();
                write!(f, "<ruby>{}</ruby>{}", entry.1, entry.0)
            }
        }
    }
}

impl std::fmt::Debug for RubyElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RubyElement::RubyText(kanji, annotation) => f
                .debug_struct("RubyText")
                .field("kanji", kanji)
                .field("annotation", annotation)
                .finish(),
            RubyElement::Ruby(map) => {
                let entry = map.iter().next().unwrap();
                f.debug_struct("Ruby")
                    .field("ruby_text", entry.1)
                    .field("base_text", entry.0)
                    .finish()
            }
        }
    }
}

fn ruby(i: &str) -> IResult<&str, RubyElement> {
    let (i, (_, rt, _, base_text)) = tuple((tag("<ruby>"), rt, tag("</ruby>"), alphanumeric))(i)?;

    Ok((i, RubyElement::new(rt, String::from(base_text))))
}

fn rt(i: &str) -> IResult<&str, RubyElement> {
    let (i, (kanji, _, annotation)) = tuple((alphanumeric, tag("<rt>"), alphanumeric))(i)?;
    Ok((
        i,
        RubyElement::RubyText(String::from(kanji), String::from(annotation)),
    ))
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
        //https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-ruby-RubyElement
        let text = "同<rt>どう";
        assert_eq!(
            rt(text),
            Ok((
                "",
                RubyElement::RubyText("同".to_string(), "どう".to_string())
            ))
        )
    }

    #[test]
    fn ruby_test() {
        let text = "<ruby>同<rt>どう</ruby>ぜず。";
        let base_text = String::from("ぜず。");
        let rt = RubyElement::RubyText("同".to_string(), "どう".to_string());
        let mut rt_dic = HashMap::new();
        rt_dic.insert(base_text, rt);
        assert_eq!(ruby(text), Ok(("", RubyElement::Ruby(rt_dic))))
    }
    #[test]
    fn serialization_test() {
        let expected = "<ruby>同<rt>どう</ruby>ぜず。";
        let dic: HashMap<String, RubyElement> = [(
            "ぜず。".to_string(),
            RubyElement::RubyText("同".to_string(), "どう".to_string()),
        )]
        .iter()
        .cloned()
        .collect();
        let ruby = RubyElement::Ruby(dic);
        assert_eq!(expected, ruby.to_string())
    }
}

fn main() {
    let text = "<ruby>同<rt>どう</ruby>ぜず。";
    println!("Parsing ruby markup annotation");
    println!("{:#?}", ruby(text));

    println!("Serialization works too");
    println!(
        "{}",
        RubyElement::new(
            RubyElement::RubyText("同".to_string(), "どう".to_string()),
            "ぜず。".to_string(),
        ),
    )
}
