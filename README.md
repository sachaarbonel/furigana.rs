# Parsing ruby markup annotation

``` rust
let text = "<ruby>同<rt>どう</ruby>ぜず。";
  println!("{:#?}", ruby(text));//will print out the ast

```

```bash
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
```

# Serialization

```rust
   let ruby = RubyElement::new(
            RubyElement::RubyText("同".to_string(), "どう".to_string()),
            "ぜず。".to_string(),
        );
    ruby.to_string();

```

```
<ruby>同<rt>どう</ruby>ぜず。
```
