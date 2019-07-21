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


}






#[derive(Parser)]
#[grammar="pest/tiger.pest"]
pub struct TigerParser;

mod token;

#[derive(Debug)]
//TODO: complete the ParserErr definition
pub enum ParserErr{
    NotImplementedErr,//special error for the unimplemented
    Expecting(Rule)
}


//Our error type
pub type ParseResult=Result<exp,ParserErr>;
use ParserErr::*;


//parse the whole program
pub fn run(src:String)->ParseResult{
    Err(NotImplementedErr)
}


