# Parsing ruby markup annotation

``` rust
let text = "<ruby>同<rt>どう</ruby>ぜず。";
  println!("{:#?}", ruby(text));//will print out the ast

```

# Serialization

```rust
  let dic: HashMap<String, RubyElement> = [(
        "ぜず。".to_string(),
        RubyElement::RubyText("同".to_string(), "どう".to_string()),
    )]
    .iter()
    .cloned()
    .collect();
    let ruby = RubyElement::Ruby(dic);
    ruby.to_string();

```
Ok(
    (
        "",
        Ruby {
            ruby_text: RubyText {
                kanji: "同",
                annotation: "どう",
            },
            base_text: "ぜず。",
        },
    ),
)
Serialization works too
<ruby>同<rt>どう</ruby>ぜず。