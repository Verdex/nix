
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
                ['t', 'y', 'p', 'e', x, rest @ .. ] if x.is_whitespace() => {
                    let (type_statement, new_input) = parse_type(rest)?; 
                    type_statements.push( type_statement );
                    input = new_input
                }
                ['f', 'u', 'n', x, rest @ .. ] if x.is_whitespace() => {
                    let (fun_statement, new_input) = parse_fun(rest)?; 
                    fun_statements.push( fun_statement );
                    input = new_input
                }
                _ => panic!( "Unknown input sequence in top level parser" ),  // TODO need to return error
            }
        }
        Ok(Module { use_statements, fun_statements, type_statements })
    }

    p(&input.chars().collect::<Vec<char>>())
}

fn parse_use( input : &[char] ) -> Result<(Use, &[char]), String> {
    Err("blah".to_string())
}

fn parse_type( input : &[char] ) -> Result<(Type, &[char]), String> {
    Err("blah".to_string())
}

fn parse_fun( input : &[char] ) -> Result<(Fun, &[char]), String> {
    Err("blah".to_string())
}
