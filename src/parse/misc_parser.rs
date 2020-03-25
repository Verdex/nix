
use super::ast::Symbol;

pub fn clear_whitespace( mut input : &[char] ) -> &[char] {
    loop {
        match input {
            [] => return input,
            [x, rest @ ..] if x.is_whitespace() => input = rest,
            _ => return input,
        }
    }
}

pub fn parse_symbol( mut input : &[char] ) -> Result<(Symbol, &[char]), String> {

    let mut ret : Vec<char> = vec![];
    
    match input {
        [] => return Err( "Expected symbol found end of file".to_string() ),
        [x, rest @ ..] if x.is_alphabetic() => {
            ret.push(*x);
            input = rest
        },
        ['_', rest @ ..] => {
            ret.push('_');
            input = rest
        },
        _ => return Err( "Encountered non symbol character".to_string() ), 
    }

    loop {
        match input {
            [] => return Err( "Expected symbol found end of file".to_string() ),
            [x, rest @ ..] if x.is_alphanumeric() => {
                ret.push(*x);
                input = rest
            },
            ['_', rest @ ..] => {
                ret.push('_');
                input = rest
            },
            _ => return Ok( (Symbol{ value : ret.into_iter().collect::<String>() }, input) ),
        }
    }
}

#[cfg(test)]
mod Test {
    use super::*;

    #[test]
    fn should_parse_symbol() {
        let input = "symbol ".chars().collect::<Vec<char>>();
        let (sym, _) = parse_symbol(&input).unwrap();
        assert_eq!( sym.value, "symbol".to_string()  );
    }

    #[test]
    fn should_parse_symbol_with_numbers() {
        let input = "s123ymbol ".chars().collect::<Vec<char>>();
        let (sym, _) = parse_symbol(&input).unwrap();
        assert_eq!( sym.value, "s123ymbol".to_string()  );
    }

    #[test]
    fn should_parse_symbol_with_underscore() {
        let input = "_sym_bol_ ".chars().collect::<Vec<char>>();
        let (sym, _) = parse_symbol(&input).unwrap();
        assert_eq!( sym.value, "_sym_bol_".to_string()  );
    }
}
