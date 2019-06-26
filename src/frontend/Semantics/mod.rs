//! The semantics module consists of semantic analyzer and Symbol table definitions.
//! Functionality such as convert a string to a symbol can be obtained here.

#[cfg(test)]
mod tests{
    use crate::frontend::Semantics::SymbolTable;

    #[test]
    fn symbol_identity(){
        //this tests that two different string slice will get different symbols.
        //And ensures the symbol is not determined by address.
        let def="def";
        let function="function";

        let mut key_tbl=SymbolTable::new();

        let def_symbol=key_tbl.get_symbol(def);
        let function_symbol=key_tbl.get_symbol(function);
        let def_symbol_duplicate=key_tbl.get_symbol("def");

        assert_ne!(def_symbol,function_symbol,"Different words mapped to a same symbol");

        assert_eq!(def_symbol,def_symbol_duplicate,"Map not stable")
    }
}


use std::collections::HashMap;

//rename i32 to Symbol
pub type Symbol=u32;

pub struct SymbolTable<'a>{
    tbl:HashMap<&'a str,Symbol>,
    count:usize
}


//formal lifetime,unbounded
impl<'a> SymbolTable<'a>{


    //create an empty symbol table
    pub fn new()->Self{
        SymbolTable{
            tbl:HashMap::new(),
            count:0
        }
    }



    //obtain a symbol from a symbol table
    pub fn get_symbol(&mut self, string:&'a str) ->Symbol{
        match self.tbl.get(string){
            Some(&i)=>i,//copy the symbol out
            None=>{
                self.tbl.insert(string,self.count as u32);
                self.count+=1;
                (self.count-1) as u32
            }
        }
    }

}