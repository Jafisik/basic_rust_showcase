
pub fn calculator(input: &str) -> Result<(), &str>  {
    let mut stack = Vec::new();
    let mut last_token = None;

    for ch in input.chars(){
        match ch{
            '(' => stack.push('('),
            ')' => {
                if stack.pop() != Some('('){
                    return Err("Neodpovídajicí závorka");
                }
            },
            '+' | '-' | '*' | '/' => {
                if last_token == Some('+') || last_token == Some('*') {
                    return Err("Dva operátory za sebou");
                }
            },
            '0'..='9' | ' ' => {  },
            _ => return Err("Nepovolený znak"),
        }
        last_token = Some(ch);
    }

    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    let mut num_buffer = String::new();
    for ch in input.chars() {
        if ch.is_digit(10) {
            num_buffer.push(ch);
        } else {
            if !num_buffer.is_empty() {
                output.push(num_buffer.clone());
                num_buffer.clear();
            }

            match ch {
                ' ' => continue,
                '(' => operators.push(ch),
                ')' => {
                    while let Some(op) = operators.pop() {
                        if op == '(' {
                            break;
                        }
                        output.push(op.to_string());
                    }
                },
                '+' | '-' | '*' | '/' => {
                    while let Some(&op) = operators.last() {
                        if precedence(op) >= precedence(ch) {
                            output.push(operators.pop().unwrap().to_string());
                        } else {
                            break;
                        }
                    }
                    operators.push(ch);
                },
                _ => return Err("Nepovolený znak při převodu"),
            }
        }
    }

    if !num_buffer.is_empty() {
        output.push(num_buffer);
    }

    while let Some(op) = operators.pop() {
        if op == '(' {
            return Err("Neuzavřená závorka při převodu");
        }
        output.push(op.to_string());
    }

    let mut eval_stack: Vec<f64> = Vec::new();
    for token in output {
        if let Ok(num) = token.parse::<f64>() {
            eval_stack.push(num);
        } else {
            let b = eval_stack.pop().ok_or("Chybějící operand")?;
            let a = eval_stack.pop().ok_or("Chybějící operand")?;
            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => return Err("Neznámý operátor"),
            };
            eval_stack.push(result);
        }
    }

    if eval_stack.len() != 1 {
        return Err("Chyba při vyhodnocení");
    }

    println!("Výsledek: {}", eval_stack[0]);
    Ok(())
}

fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}