#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    let mut a = 0;
    let mut b = 0;
    for i in 0..tokens.len() {
        match tokens[i]{
            InfixToken::Operator(op) => {
                if (tokens[i] == tokens[0]) | (tokens[i] == tokens[tokens.len()-1]) {
                    return None
                }
                if i > 0 {
                    if (tokens[i-1]==InfixToken::LeftParen) | (tokens[i-1]==InfixToken::Operator(op)) {
                        return None
                    }
                }
            }
            InfixToken::Operand(n) => {
                if i>0 {
                    match tokens[i-1]{
			InfixToken::Operator(op) => {}
			InfixToken::Operand(n) => { return None}
			InfixToken::LeftParen => {}
			InfixToken::RightParen => {return None}
		    }
                }
            }
            InfixToken::LeftParen => {
                if i>0 {
                    match tokens[i-1]{
                        InfixToken::Operand(n) => {
                            return None
                        }
                        InfixToken::RightParen =>{
                            return None
                        }
                        InfixToken::Operator(ref op) => {}
                        InfixToken::LeftParen => {}
                    }
                    if tokens[i] == tokens[tokens.len()-1] {
                        return None
                    }
                }
                a += 1;
            }
            InfixToken::RightParen => {
                if i>0 {
                    match tokens[i-1]{
                        InfixToken::Operator(ref op) => {
                            return None
                        }
                        InfixToken::LeftParen => {
                            return None
                        }
                        InfixToken::Operand(n) => {}
                        InfixToken::RightParen => {}
                    }
                }
                if tokens[i] == tokens[0] {
                    return None
                }
                b += 1;
                if b > a {
                    return None
                }
            }
        }
    }
    if a != b {
            return None
    }
    let mut output: Vec<PostfixToken> = Vec::new();
    let mut stack : Vec<InfixToken> = Vec::new(); 
    for i in tokens {
        match *i {
            InfixToken::Operator(ref op) => {
                if stack.len() > 0 {
                    let mut x = stack.pop().unwrap();
                    if (*op == Operator::Add) | (*op==Operator::Sub) {
                        while (stack.len()>=0) {
                            match x {
                                InfixToken::Operator(op) => {output.push(PostfixToken::Operator(op));}
                                InfixToken::Operand(n) => {}
                                InfixToken::LeftParen => {
                                    stack.push(InfixToken::LeftParen);
                                    stack.push(InfixToken::Operator(*op));
                                    break;
                                }
                                InfixToken::RightParen => {}
                            }
                            if stack.len() == 0 { 
                                stack.push(InfixToken::Operator(*op));
                                break;
                            }
                            x = stack.pop().unwrap();
                        }
                    }
                    if (*op == Operator::Mul) | (*op==Operator::Div) {
                        while (stack.len()>=0) {
                            if (x==InfixToken::Operator(Operator::Mul)) | (x==InfixToken::Operator(Operator::Div)) {
                                match x {
                                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                    InfixToken::Operand(n) => {}
                                    InfixToken::LeftParen => {}
                                    InfixToken::RightParen => {}
                                }
                                if stack.len() == 0 {
                                    stack.push(InfixToken::Operator(*op));
                                    break; }
                                x = stack.pop().unwrap();
                            }
                            else if (x==InfixToken::Operator(Operator::Add)) | (x==InfixToken::Operator(Operator::Sub)) {
                                match x{
                                    InfixToken::Operator(op) => stack.push(InfixToken::Operator(op)),
                                    InfixToken::Operand(n) => {}
                                    InfixToken::LeftParen => {}
                                    InfixToken::RightParen => {}
                                }
                                break;
                            }
                            else if x==InfixToken::LeftParen {
                                    match x{
                                        InfixToken::Operator(op) => {}
                                        InfixToken::Operand(n) => {}
                                        InfixToken::LeftParen => stack.push(InfixToken::LeftParen),
                                        InfixToken::RightParen => {}
                                    }
                                    stack.push(InfixToken::Operator(*op));
                                    break;
                            }
                            else {  
                                if stack.len()== 0 {
                                    stack.push(InfixToken::Operator(*op)); //HERE
                                    break;
                                }
                            }
                        }
                    }
                    if ((x == InfixToken::Operator(Operator::Add)) | (x==InfixToken::Operator(Operator::Sub))) &
                       ((*op == Operator::Mul) | (*op == Operator:: Div)){ 
                        stack.push(InfixToken::Operator(*op)); 
                    }
                }
                else {
                    stack.push(InfixToken::Operator(*op));
                }
            }
            InfixToken::Operand(n) => output.push(PostfixToken::Operand(n)),
            InfixToken::LeftParen => stack.push(InfixToken::LeftParen),
            InfixToken::RightParen => {
                let mut x = stack.pop().unwrap();
                while x != InfixToken::LeftParen {
                    match x {
                        InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                        InfixToken::Operand(n) => {}
                        InfixToken::LeftParen => {}
                        InfixToken::RightParen => {}
                    }
                    if x == InfixToken::LeftParen{ break; }
                    if stack.len() == 0 { break; }
                    x = stack.pop().unwrap();
                }
            }
        }
    }
    
    while stack.len() > 0 {
        let y = stack.pop().unwrap();
        match y {
            InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
            InfixToken::Operand(n) => {}
            InfixToken::LeftParen => {}
            InfixToken::RightParen => {}
        }
    }
        
    let x: Option<Vec<PostfixToken>> = Some(output);
    x
}
