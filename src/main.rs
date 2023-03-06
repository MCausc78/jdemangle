pub enum JType {
	Boolean,
	Byte,
	Character,
	Short,
	Integer,
	Long,
	Float,
	Double,
	Class(String),
	Array(Box<JType>),
	Method(Box<JType>, Vec<JType>),
	Void,
}

impl JType {
	pub fn demangle_to_java(&self) -> String {
		match self {
			JType::Boolean => "boolean".to_string(),
			JType::Byte => "byte".to_string(),
			JType::Character => "char".to_string(),
			JType::Short => "short".to_string(),
			JType::Integer => "int".to_string(),
			JType::Long => "long".to_string(),
			JType::Float => "float".to_string(),
			JType::Double => "double".to_string(),
			JType::Class(name) => name.replace('/', "."),
			JType::Array(element) => {
				let mut result = element.demangle_to_java();
				result.push_str("[]");
				result
			},
			JType::Method(return_type, args) => {
				let mut result = String::new();
				result.push_str(&return_type.demangle_to_java());
				result.push('(');
				if !args.is_empty() {
					let j = args.len();
					for i in 0..j {
						if i > 0 {
							result.push_str(", ");
						}
						result.push_str(args[i]
							.demangle_to_java()
							.as_str());
					}
				}
				result.push(')');
				result
			},
			JType::Void => "void".to_string(),
		}
	}
}

pub fn parse_signature(parsing_text: &mut String) -> Option<JType> {
	let ch = parsing_text
		.chars()
		.nth(0);
	if ch.is_none() {
		return None;
	}
	let c = ch.unwrap();
	match c {
		'Z' => Some(JType::Boolean),
		'B' => Some(JType::Byte),
		'C' => Some(JType::Character),
		'S' => Some(JType::Short),
		'I' => Some(JType::Integer),
		'J' => Some(JType::Long),
		'F' => Some(JType::Float),
		'D' => Some(JType::Double),
		'V' => Some(JType::Void),
		'L' => {
			let mut result = String::new();
			let mut last_char: char;
			parsing_text.remove(0); /* eat L */
			last_char = parsing_text.remove(0);
			while last_char != ';' {
				result.push(last_char);
				last_char = parsing_text.remove(0);
			}
			parsing_text.insert(0, last_char);
			Some(JType::Class(result))
		},
		'[' => {
			parsing_text.remove(0); /* eat [ */
			match parse_signature(parsing_text) {
				None => None,
				Some(jtype) => Some(JType::Array(Box::new(jtype))),
			}
		},
		'(' => {
			let mut args: Vec<JType> = Vec::new();
			let mut last_char: char;
			parsing_text.remove(0); /* eat ( */
			while { last_char = parsing_text
					.chars()
					.nth(0)
					.unwrap();
				last_char } != ')'
			{
				let signature = parse_signature(parsing_text);
				if signature.is_some() {
					args.push(signature.unwrap());
				} else {
					return None;
				}
				parsing_text.remove(0);
			}
			parsing_text.remove(0); /* eat ) */
			match parse_signature(parsing_text) {
				None => None,
				Some(jtype) => Some(JType::Method(Box::new(jtype), args))
			}
		}
		_ => None,
	}
}

fn main() {
	let mut signature = String::new();
	let stdin = std::io::stdin();
	while {
		if let Ok(count) = stdin.read_line(&mut signature) {
			count > 0
		} else {
			false
		}
	} {
		match parse_signature(&mut signature) {
			Some(jtype) => println!("{}", jtype.demangle_to_java()),
			None => println!("{}", signature),
		}
		signature.clear();
	}
}
