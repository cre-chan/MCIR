use crate::frontend::Semantics::Symbol;

//created so as to simplify the list definition
enum List<T>{
    Cons(T,Box<Self>),
    Nil
}

//aliases for cons list, corresponds to sequence of expressions
pub type efieldList=List<efield>;
pub type NameTyList=List<namety>;
pub type declist=List<decl>;
pub type fundeclist=List<fundec>;
pub type fieldlist=List<field>;
pub type explist=List<exp>;


//! syntactical structure for the left values.
pub enum lvalue{
    Simple{sym:Symbol},//sym
    Field{var:Box<lvalue>,field:Symbol},//var.field
    Subscript{var:Box<lvalue>,index:Box<exp>}//var[index]
}

//! the main structure of tiger lang
pub enum exp{
    Var{var:Box<lvalue>},//var
    Nil,//Nil
    Int{i:i64},//i
    String{s:Symbol},//s
    Call{func:Symbol,args:Box<explist>},//func(args)
    Op{oper: Operator,lop:Box<exp>,rop:Box<exp>},//lop op rop
    Record{typ:Symbol,fields:Box<efieldList>},//typ{fields}
    Seq{seq:Box<explist>},//explist, a sequence of consecutive expressions
    Assign{var:Box<lvalue>,expr:Box<exp>},//var:=Box<exp>
    If{test:Box<exp>,then:Box<exp>,els:Box<exp>},//if test then Box<exp> else Box<exp>
    While{test:Box<exp>,body:Box<exp>},//while test {Box<exp>}
    Break,
    For{var:Symbol,lo:Box<exp>,hi:Box<exp>,body:Box<exp>},//for var in lo..hi {Box<exp>}
    Let{decs:Box<declist>,body:Box<exp>},//let declist in body
    Array{typ:Symbol,size:Box<exp>,init:Box<exp>}//
}

//! the declaration of functions, variables and types are divided into consecutive blocks.
//! relative declarations are placed together.
pub enum decl{
    Func{function:fundeclist},//consecutive definition of functions
    Var{var:Symbol,typ:Symbol,init:Box<exp>},//var var:typ=init
    Type{typs:NameTyList}//consecutive definition of types
}

//! corresponds to type variable
pub enum typ{
    Namely{name:Symbol},//alias for a type
    Record{fields:fieldlist},//record type
    Array{array:Symbol}
}

//! basic part of struct definition
pub struct field{ name:Symbol,typ:Symbol }//name:typ

//! function definition
pub struct fundec{
    name:Symbol,
    params:fieldlist,
    result:Symbol,
    body:exp
}

//! connect a symbol with a type
pub struct namety{
    name:Symbol,
    ty:typ
}

//! basic parts of a structure value
pub struct  efield{name:Symbol,expr:exp}

//! namely.
pub enum Operator {
    plus,
    minus,
    times,
    div,
    eq,
    neq,
    lt,
    le,
    gt,
    ge
}

