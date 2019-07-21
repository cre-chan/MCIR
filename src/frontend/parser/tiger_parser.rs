use pest::Parser;
use pest::iterators::*;
use std::fmt::Debug;



use super::AST::*;


#[cfg(test)]
mod tests{
    use std::fs::File;
    use std::io::Read;
    use super::*;
    use crate::frontend::parser::tiger_parser::TigerParser;

    //the function is used to parse a src according to the specified rule
    fn parse_src(src:&str,rule:Rule)->Pair<Rule>{
        TigerParser::parse(rule,src).expect("source string formatting error")
            .next()
            .unwrap()
    }

    #[test]
    //this tests if the starting rule is program
    fn starting_rule(){
        let mut src=File::open(
            "test_examples/HelloWorld.tiger"
        )
            .expect("File path error");
        let mut src_string=String::new();

        src.read_to_string((&mut src_string));

        let mut program=TigerParser::parse(Rule::program,&src_string)
            .expect("Parsing failed");
        let program=program.next().expect("No program");
        assert_eq!(program.as_rule(),Rule::program);
    }


    #[test]
    fn string_literal_parsing_test(){
        let string=parse_src("\"ab\"",Rule::string);

        assert_eq!(expect_string(string).unwrap(),"ab")
    }


    #[test]
    fn inner_parsing_test(){
        let inner=parse_src("abcd",Rule::inner);

        assert_eq!(expect_inner(inner).unwrap(),"abcd")
    }


    #[test]
    fn interger_parsing_test(){
        let int=parse_src("123456",Rule::interger);
        let int=expect_interger(int).unwrap();

        assert_eq!(int,123456)
    }


    #[test]
    fn identifier_test(){
        let src="ascii_code123";
        let id=TigerParser::parse(Rule::identifier,src)
            .expect("Parsing failed")
            .next()
            .unwrap();

        let id_name=expect_identifier(id)
            .unwrap();

        assert_eq!(id_name,String::from("ascii_code123"));
    }


    // Tests for parsing escape characters
    #[test]
    fn escape_test(){
        let src="\\045";
        let id=TigerParser::parse(Rule::char,src)
            .expect("Parsing failed")
            .next()
            .unwrap();

        let id_name=transform_ch(id)
            .unwrap();

        assert_eq!(id_name,String::from("%"));
    }


    #[test]
    fn normal_char_test(){
        let src="a";
        let id=TigerParser::parse(Rule::char,src)
            .expect("src format error")
            .next()
            .unwrap();

        let id_name=transform_ch(id).unwrap();

        assert_eq!(id_name,String::from("a"))
    }


    #[test]
    #[should_panic("Expecting char. Identifier found")]
    //the transform_ch distinguishes char from other
    fn not_a_character(){
        let src="ascii_code123";
        let id=TigerParser::parse(Rule::identifier,src)
            .expect("Parsing failed")
            .next()
            .unwrap();

        let id_name=transform_ch(id)
            .expect("Expecting char. Identifier found");
    }


    #[test]
    fn compare_op_parsing_test(){
        let src=["=","<>",">","<",">=","<="];
        let compare_ops:Vec<Operator>=src.iter()
            .map(
                |&str| TigerParser::parse(Rule::compareop,str)
                    .expect("src format error")
                    .next()
                    .unwrap()
            ).map(
            |r| expecting_compareop(r)
        ).collect();

        assert_eq!(
            compare_ops,
            vec![Operator::Eq,Operator::Neq,Operator::GT,Operator::LT,Operator::GE,Operator::LE]
        )
    }

    #[test]
    fn LE_parsing_test(){
        let le=parse_src("<=",Rule::compareop);
        let le=expecting_compareop(le);

        assert_eq!(le,Operator::LE)
    }

    #[test]
    fn matching_test(){
        let le="<=";

        assert_eq!(
            match le{
                "<="=>1,
                "<"=>2,
                _=>3
            },
            1
        )
    }
}






#[derive(Parser)]
#[grammar="pest/tiger.pest"]
pub struct TigerParser;


#[derive(Debug)]
//TODO: complete the ParserErr definition
pub enum ParserErr{
    NotImplementedErr,//special error for the unimplemented
    Expecting(Rule)
}


//Our error type
pub type ParseResult=Result<exp,ParserErr>;
use ParserErr::*;
use pest::RuleType;


//parse the whole program
pub fn run(src:String)->ParseResult{
    Err(NotImplementedErr)
}


//transform a
fn expect_identifier(id:Pair<Rule>)->Result<String,ParserErr>{
    if let Rule::identifier=id.as_rule(){
        Ok(String::from(id.as_str()))
    }else{
        Err(Expecting(Rule::identifier))
    }
}


fn expect_interger(int:Pair<Rule>)->Result<u32,ParserErr>{
    if let Rule::interger=int.as_rule(){
        Ok(int
            .as_str()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .fold(0,|sum,entry| sum*10+entry)
        )
    }else{
        Err(Expecting(Rule::interger))
    }
}


fn expect_string(string:Pair<Rule>)->Result<String,ParserErr>{
    if let Rule::string=string.as_rule(){
        expect_inner(string.into_inner().next().unwrap())
    }else{
        Err(Expecting(Rule::string))
    }
}


fn expect_inner(inner:Pair<Rule>)->Result<String,ParserErr>{
    if let Rule::inner=inner.as_rule(){
        Ok(
            inner.into_inner()
                .map(|pair|transform_ch(pair).unwrap())
                .fold(
                    String::new(),
                      |sum,entry|sum+&entry
                )
        )
    }else{
        Err(Expecting(Rule::inner))
    }
}


///parse escape characters into str literal
fn transform_ch(c:Pair<Rule>)->Result<String,ParserErr>{
    if let Rule::char=c.as_rule(){
        match c.as_str(){
            "\\\""=>Ok(String::from("\"")),
            "\\\\"=>Ok(String::from("\\")),
            "\\n"=>Ok(String::from("\n")),
            "\\t"=>Ok(String::from("\t")),
            c=>{
                if c.starts_with("\\"){
                    let oct=c[1..].chars()
                        .map(|c| c.to_digit(8).unwrap())
                        .fold(0,|sum,entry| sum*8+entry) as u8;

                    let mut oct_char=String::new();
                    oct_char.push(oct as char);
                    Ok(oct_char)

                }else{
                    Ok(String::from(c))
                }
            }
        }
    }else{
        Err(Expecting(Rule::char))
    }
}


// Parse a comparison operator
fn expecting_compareop(compareop:Pair<Rule>)->Operator{
    if let Rule::compareop=compareop.as_rule(){
        match compareop.as_str()  {
            "="=>Operator::Eq,
            "<>"=>Operator::Neq,
            "<="=>Operator::LE,
            "<"=>Operator::LT,
            ">="=>Operator::GE,
            ">"=>Operator::GT,
            _=>panic!("Internal error,{} found",compareop.as_str())
        }
    }else{
        panic!("Expecting {:?}, found {:?}",Rule::compareop,compareop.as_rule());
    }
}
