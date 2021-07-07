use std::any::type_name;

pub fn split_left(text: &str, delimiter: &str) -> Option<String> {
    println!("Searching for '{}' in: {}", delimiter, text );
    let chunks : Vec<&str> = text.split( delimiter ).collect();

    if chunks.len() > 1 {
	Some( chunks.first().unwrap().to_string() )
    } else {
	None
    }
}

pub fn split_right(text: &str, delimiter: &str) -> Option<String> {
    println!("Searching for '{}' in: {}", delimiter, text );
    let chunks : Vec<&str> = text.split( delimiter ).collect();

    if chunks.len() > 1 {
	Some( chunks.last().unwrap().to_string() )
    } else {
	None
    }
}

pub fn type_of<T: std::fmt::Debug>(v: &T) -> String {
    let name = String::from( type_name::<T>() );
    println!("Type of '{:?}': {}", v, name );
    split_right( &name, ":" ).unwrap_or(
	"Unknown".to_string()
    )
}

pub fn struct_name<T: std::fmt::Debug>(v: &T) -> String {
    println!("Struct name from: {:?}", v );
    let repr = format!("{:?}", v );

    let repr = split_left( &repr, " ").unwrap_or(repr);

    if &repr[..1] == "[" {
	return "Error".into();
    }

    split_left( &repr, "(" ).unwrap_or(
	split_left( &repr, " {" ).unwrap_or(
	    repr
	)
    )
}
