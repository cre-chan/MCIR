use crate::frontend::Semantics::Symbol;
use std::fmt::Debug;


#[derive(Debug)]
//created so as to simplify the list definition. Not recommended.
pub enum List<T>{
    Cons(T,Box<Self>),
    Nil
}

//aliases for cons list, corresponds to sequence of expressions
pub type EfieldList =List<efield>;
pub type NameTyList=List<namety>;
pub type Declist =List<decl>;
pub type Fundeclist =List<fundec>;
pub type Fieldlist =List<field>;
pub type Explist =List<exp>;


#[derive(Debug)]
/// syntactical structure for the left values.
pub enum lvalue{
    Simple{sym:Symbol},//sym
    Field{var:Box<lvalue>,field:Symbol},//var.field
    Subscript{var:Box<lvalue>,index:Box<exp>}//var[index]
}


#[derive(Debug)]
/// the main structure of tiger lang
pub enum exp{
    Var{var:Box<lvalue>},//var
    Nil,//Nil
    Int{i:i64},//i
    String{s:Symbol},//s
    Call{func:Symbol,args:Box<Explist>},//func(args)
    Op{oper: Operator,lop:Box<exp>,rop:Box<exp>},//lop op rop
    Record{typ:Symbol,fields:Box<EfieldList>},//typ{fields}
    Seq{seq:Box<Explist>},//Explist, a sequence of consecutive expressions
    Assign{var:Box<lvalue>,expr:Box<exp>},//var:=Box<exp>
    If{test:Box<exp>,then:Box<exp>,els:Box<exp>},//if test then Box<exp> else Box<exp>
    While{test:Box<exp>,body:Box<exp>},//while test {Box<exp>}
    Break,
    For{var:Symbol,lo:Box<exp>,hi:Box<exp>,body:Box<exp>},//for var in lo..hi {Box<exp>}
    Let{decs:Box<Declist>,body:Box<exp>},//let Declist in body
    Array{typ:Symbol,size:Box<exp>,init:Box<exp>}//
}


#[derive(Debug)]
/// the declaration of functions, variables and types are divided into consecutive blocks.
/// relative declarations are placed together.
pub enum decl{
    Func{function: Fundeclist },//consecutive definition of functions
    Var{var:Symbol,typ:Symbol,init:Box<exp>},//var var:typ=init
    Type{typs:NameTyList}//consecutive definition of types
}


#[derive(Debug)]
/// corresponds to type variable
pub enum typ{
    Namely{name:Symbol},//alias for a type
    Record{fields: Fieldlist },//record type
    Array{array:Symbol}
}


#[derive(Debug)]
/// basic part of struct definition
pub struct field{ name:Symbol,typ:Symbol }//name:typ


#[derive(Debug)]
/// function definition
pub struct fundec{
    name:Symbol,
    params: Fieldlist,
    result:Symbol,
    body:exp
}


#[derive(Debug)]
/// connect a symbol with a type
pub struct namety{
    name:Symbol,
    ty:typ
}


#[derive(Debug)]
/// basic parts of a structure value
pub struct  efield{name:Symbol,expr:exp}


#[derive(Debug)]
#[derive(PartialEq)]
/// namely.
pub enum Operator {
    Plus,
    Minus,
    Times,
    Div,
    Eq,
    Neq,
    LT,
    LE,
    GT,
    GE
}

