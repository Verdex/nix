
use super::ast::*;

pub fn parse( input : &str ) -> Result<Module, String> {
    fn p( mut input : &[char] ) -> Result<Module, String> {
        let mut use_statements : Vec<Use> = vec![];
        let mut fun_statements : Vec<Fun> = vec![];
        let mut type_statements : Vec<Type> = vec![];
        loop {
            match input {
                [] => break,
                [x, rest @ .. ] if x.is_whitespace() => input = rest,
                ['u', 's', 'e', x, rest @ .. ] if x.is_whitespace() => { 
                    let (use_statement, new_input) = parse_use(rest)?; 
                    use_statements.push( use_statement );
                    input = new_input
                },
                _ => println!("else"),
            }
        }
        Ok(Module { use_statements, fun_statements, type_statements })
    }

    p(&input.chars().collect::<Vec<char>>())
}

fn parse_use( input : &[char] ) -> Result<(Use, &[char]), String> {
    Err("blah".to_string())
}
