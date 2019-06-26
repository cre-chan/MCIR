//! Core crate for the compiler.Uses the pest parser generator to generate paser.


extern crate pest;
#[macro_use]
extern crate pest_derive;


//the frontend of MCIR
//lexer is deprecated and only parsing and semantic analysis are left
mod frontend;
mod backend;
