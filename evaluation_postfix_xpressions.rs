pub enum Operator {
	//'+'
	Add,
	//'-'
	Sub,
	//'*'
	Mul,
}

pub enum Token {
	Operator(Operator),
	Operand(isize),
}


//Evaluates postfix expression
//Input: postfix expression, where each element contains operator or operand.
//Returns: if postfix expression is valid, returns 'Some(value)';
// 	otherwise returns 'None'
pub fn eval(tokens: &[Token]) -> Option<isize> {
	let mut  v: Vec<isize> = Vec::new();
	for i in tokens {
		match i {
			&Token::Operator(ref op) => {
				if v.len()>1 { 
					let x = v.pop().unwrap();
					let y = v.pop().unwrap();
					match op {
						&Operator::Add => v.push(x+y),
						&Operator::Sub => v.push(y-x),
						&Operator::Mul => v.push(x*y),
					};
				} else {
					return None;
				}
			},
			&Token::Operand(n) => v.push(n),
		};
	}
	if v.len()==1 {
		return v.pop()
	} else {
		return None
	}

}


