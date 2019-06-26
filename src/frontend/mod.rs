#[cfg(test)]
mod tests{


    #[test]
    fn it_works(){
        println!("Frontend test is ready")
    }
}




#[allow(non_snake_case)]
pub mod Semantics;//module for semantic analysis
pub mod parser;//parse the source code and pass the AST to semantics module