#[cfg(test)]
mod tests{
    use std::fs::File;
    use std::io::Read;
    use super::*;
    use pest::Parser;
    use crate::frontend::parser::tiger_parser::TigerParser;

    //the function is used to parse a src according to the specified rule
    fn parse_src(src:&str,rule:Rule)->Pair<Rule>{
        TigerParser::parse(rule,src).expect("source string formatting error")
            .next()
            .unwrap()
    }



    #[test]
    fn string_literal_parsing_test(){
        let string=parse_src("\"ab\"",Rule::string);

        assert_eq!(expect_string(string),"ab")
    }


    #[test]
    fn inner_parsing_test(){
        let inner=parse_src("abcd",Rule::inner);

        assert_eq!(expect_inner(inner),"abcd")
    }


    #[test]
    fn interger_parsing_test(){
        let int=parse_src("123456",Rule::interger);
        let int=expect_interger(int);

        assert_eq!(int,123456)
    }


    #[test]
    fn identifier_test(){
        let src="ascii_code123";
        let id=TigerParser::parse(Rule::identifier,src)
            .expect("Parsing failed")
            .next()
            .unwrap();

        let id_name=expect_identifier(id);

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

        let id_name=transform_ch(id);

        assert_eq!(id_name,String::from("%"));
    }


    #[test]
    fn normal_char_test(){
        let src="a";
        let id=TigerParser::parse(Rule::char,src)
            .expect("src format error")
            .next()
            .unwrap();

        let id_name=transform_ch(id);

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

        let id_name=transform_ch(id);
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


    #[test]
    fn plusandminus_parsing_test(){

    }

}

use super::ParserErr::{self,*};
use pest::iterators::*;
use crate::frontend::parser::AST::Operator;
use super::Rule;


//transform a
pub fn expect_identifier(id:Pair<Rule>)->String{
    if let Rule::identifier=id.as_rule(){
        String::from(id.as_str())
    }else{
        panic!("Expecting Identifier, found {:?}",id.as_rule())
    }
}


pub fn expect_interger(int:Pair<Rule>)->u32{
    if let Rule::interger=int.as_rule(){
        int.as_str()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .fold(0,|sum,entry| sum*10+entry)
    }else{
        panic!("Expecting Interger, found {:?}",int.as_rule())
    }
}


pub fn expect_string(string:Pair<Rule>)->String{
    if let Rule::string=string.as_rule(){
        expect_inner(string.into_inner().next().unwrap())
    }else{
        panic!("Expecting String, found {:?}",string.as_rule())
    }
}


pub fn expect_inner(inner:Pair<Rule>)->String{
    if let Rule::inner=inner.as_rule(){
        inner.into_inner()
            .map(|pair|transform_ch(pair))
            .fold(
                String::new(),
                |sum,entry|sum+&entry
            )
    }else{
        panic!("Expecting Inner, found {:?}",inner.as_rule())
    }
}


///parse escape characters into str literal
pub fn transform_ch(c:Pair<Rule>)->String{
    if let Rule::char=c.as_rule(){
        match c.as_str(){
            "\\\""=>String::from("\""),
            "\\\\"=>String::from("\\"),
            "\\n"=>String::from("\n"),
            "\\t"=>String::from("\t"),
            c=>{
                if c.starts_with("\\"){
                    let oct=c[1..].chars()
                        .map(|c| c.to_digit(8).unwrap())
                        .fold(0,|sum,entry| sum*8+entry) as u8;

                    let mut oct_char=String::new();
                    oct_char.push(oct as char);
                    oct_char

                }else{
                    String::from(c)
                }
            }
        }
    }else{
        panic!("Expecting Character, found {:?}",c.as_rule())
    }
}


// Parse a comparison operator
pub fn expecting_compareop(compareop:Pair<Rule>)->Operator{
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


//TODO: ADD test for this function
pub fn expecting_plusorminus(plus_or_minus:Pair<Rule>) ->Operator{
    if let Rule::plusorminus= plus_or_minus.as_rule(){
        match plus_or_minus.as_str(){
            "+"=>Operator::Plus,
            "-"=>Operator::Minus,
            _=>panic!("Internal error,{} found", plus_or_minus.as_str())
        }
    }else{
        panic!("Expecting {:?},found {:?}", Rule::plusorminus, plus_or_minus.as_rule())
    }
}


#[allow()]
//TODO: add test for this function
pub fn expecting_timesordiv(timesordiv:Pair<Rule>)->Operator{
    if let Rule::timesordiv=timesordiv.as_rule(){
        match timesordiv.as_str(){
            "+"=>Operator::Plus,
            "-"=>Operator::Minus,
            _=>panic!("Internal error,{} found",timesordiv.as_str())
        }
    }else{
        panic!("Expecting {:?},found {:?}",Rule::timesordiv,timesordiv.as_rule())
    }
}