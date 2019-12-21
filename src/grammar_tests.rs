use crate::grammar;

#[test]
fn test_numbers() {
    assert!(grammar::ObjectParser::new().parse("{x=42}").is_ok());
    assert!(grammar::ObjectParser::new().parse("{x=-42}").is_ok());
    assert!(grammar::ObjectParser::new().parse("{x=0}").is_ok());
    assert!(grammar::ObjectParser::new().parse("{x=-1}").is_ok());
    assert!(grammar::ObjectParser::new()
        .parse("{x=-122.532232}")
        .is_ok());
}
#[test]
fn test_strings() {
    assert!(grammar::ObjectParser::new()
        .parse(r#"{x="testString"}"#)
        .is_ok());
    assert!(grammar::ObjectParser::new().parse(r#"{x=""}"#).is_ok());
}

#[test]
fn test_objects() {
    assert!(grammar::ObjectParser::new().parse(r#"{}"#).is_ok());
    assert!(grammar::ObjectParser::new()
        .parse("{speed=0,productivity=0,consumption=0}")
        .is_ok());
    assert!(grammar::ObjectParser::new().parse("{a=0,a=0}").is_ok());
    assert!(grammar::ObjectParser::new().parse("{a=0}").is_ok());
    assert!(grammar::ObjectParser::new().parse(r#"{0=""}"#).is_ok());
}

#[test]
fn test_arrays() {
    assert!(grammar::ObjectParser::new().parse(r#"{a=[]}"#).is_ok());
    assert!(grammar::ObjectParser::new().parse(r#"{a=[1]}"#).is_ok());
    assert!(grammar::ObjectParser::new()
        .parse(r#"{a=[1,2,3,4]}"#)
        .is_ok());
    assert!(grammar::ObjectParser::new()
        .parse(r#"{a=["a","b"]}"#)
        .is_ok());
    assert!(grammar::ObjectParser::new()
        .parse(r#"{a=[[],[[]] ]}"#)
        .is_ok());
    assert!(grammar::ObjectParser::new()
        .parse(r#"{[{a="b"}]="a"}"#)
        .is_ok());
    assert!(grammar::ObjectParser::new()
        .parse(r#"{["a","b"]=["c"]}"#)
        .is_ok());
    assert!(grammar::ObjectParser::new().parse(r#"{["a"]={}}"#).is_ok());
}
