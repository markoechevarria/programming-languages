fn main() {
    // 1
    let s: &str = "hello world";
    println!("{s}");

    // 2
    let s: Box<str> = "hello world".into();
    greetings(&s);

    // 3
    let  mut s = String::new();
    s.push_str("hello world");
    s.push('!');

    assert_eq!(s, "hello world!");

    // 4
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";
    println!("{}", s);

    // 5
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");
    assert_eq!(s1, "I like cats");

    // 6
    
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");

    // 7
    let s = "hello world".to_string();
    greetings2(s);

    // 8
    let s = "hello, world".to_string();
    let _s1: &str = &s;

    // 9
    let byte_escape = "Im writing Ru\x73t!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);

    // 10
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    // 11
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);


    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb";

    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };

    // 12
    let s1 = String::from("hi,中国");
    let h = &s1[0..1];
    assert_eq!(h, "h");

    let h1 = &s1[3..6];
    assert_eq!(h1, "中");

    // 13
    for c in "你好，世界".chars() {
        println!("{}", c)
    }

    // 14
    // let s = "The 🚀 goes to the 🌑!";
    // let rocket = utf8_slice::slice(s, 4, 5);
    
}

fn greetings(s: &str) {
    println!("{s}");
}

fn greetings2(s: String) {
    println!("{}", s)
}
