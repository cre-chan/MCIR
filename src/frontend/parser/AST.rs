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
pub type Explist =List<Expr>;


#[derive(Debug)]
#[allow(non_camel_case_types)]
/// syntactical structure for the left values.
pub enum lvalue{
    Simple{sym:Symbol},//sym
    Field{var:Box<lvalue>,field:Symbol},//var.field
    Subscript{var:Box<lvalue>,index:Box<Expr>}//var[index]
}


#[derive(Debug)]
/// the main structure of tiger lang
pub enum Expr {
    Var{var:Box<lvalue>},//var
    Nil,//Nil
    Int{i:i64},//i
    String{s:Symbol},//s
    Call{func:Symbol,args:Box<Explist>},//func(args)
    Op{oper: Operator,lop:Box<Expr>,rop:Box<Expr>},//lop op rop
    Record{typ:Symbol,fields:Box<EfieldList>},//typ{fields}
    Seq{seq:Box<Explist>},//Explist, a sequence of consecutive expressions
    Assign{var:Box<lvalue>,expr:Box<Expr>},//var:=Box<exp>
    If{test:Box<Expr>,then:Box<Expr>,els:Box<Expr>},//if test then Box<exp> else Box<exp>
    While{test:Box<Expr>,body:Box<Expr>},//while test {Box<exp>}
    Break,
    For{var:Symbol,lo:Box<Expr>,hi:Box<Expr>,body:Box<Expr>},//for var in lo..hi {Box<exp>}
    Let{decs:Box<Declist>,body:Box<Expr>},//let Declist in body
    Array{typ:Symbol,size:Box<Expr>,init:Box<Expr>}//
}


#[derive(Debug)]
/// the declaration of functions, variables and types are divided into consecutive blocks.
/// relative declarations are placed together.
pub enum decl{
    Func{function: Fundeclist },//consecutive definition of functions
    Var{var:Symbol,typ:Symbol,init:Box<Expr>},//var var:typ=init
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
    body: Expr
}


#[derive(Debug)]
/// connect a symbol with a type
pub struct namety{
    name:Symbol,
    ty:typ
}


#[derive(Debug)]
/// basic parts of a structure value
pub struct  efield{name:Symbol,expr: Expr }


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

