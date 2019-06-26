

use pest::Parser;

#[derive(Parser)]
#[grammar="./pest/tiger.pest"]
struct TigerParser;