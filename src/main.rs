use std::env::args;

#[derive(Debug)]
#[allow(unused)]
enum Ops {
    Add,
    Multiply,
    Subtract,
    Divide,
    LeftParen,
    RightParen,
}

impl Ops {
    fn value(&self) -> i8 {
        match *self {
            Ops::LeftParen => 0,
            Ops::RightParen => 0,
            Ops::Subtract => 1,
            Ops::Add => 1,
            Ops::Multiply => 2,
            Ops::Divide => 2,
        }
    }

    fn into_char(&self) -> char {
        match *self {
            Ops::Add => '+',
            Ops::Subtract => '-',
            Ops::Divide => '/',
            Ops::Multiply => '*',
            Ops::LeftParen => '(',
            Ops::RightParen => ')',
        }
    }
}

fn check_precedence(op_stack: &mut Vec<Ops>, operator: Ops, final_stack: &mut String) {
    if op_stack.len() <= 1 {
        op_stack.push(operator);
        return;
    }
    while op_stack.last().unwrap().value() < operator.value() {
        if op_stack.len() <= 1 {
            break;
        }
        final_stack.push(op_stack.pop().unwrap().into_char());
    }

    op_stack.push(operator);
}

fn eval(eq: String) -> i32 {
    let mut int_stack: Vec<i32> = vec![];

    let mut buffer: char;

    for i in eq.chars() {
        buffer = i;

        if buffer.is_numeric() {
            let num_str = buffer.to_string();
            int_stack.push(num_str.parse().expect("Not a digit"));
        } else if buffer == ' ' {
            continue;
        } else {
            let num2 = int_stack.pop().unwrap();
            let num1 = int_stack.pop().unwrap();

            match buffer {
                '*' => int_stack.push(num1 * num2),
                '-' => int_stack.push(num1 - num2),
                '+' => int_stack.push(num1 + num2),
                '/' => int_stack.push(num1 / num2),
                _ => {
                    continue;
                }
            }
        }
    }
    int_stack.pop().unwrap()
}

fn main() {
    let argv: Vec<String> = args().collect();

    if argv.len() < 2 {
        panic!("An equation was not provided");
    }

    let main_eq: String = argv[1].trim().replace(' ', "").into();
    let mut i = 0;

    let mut final_stack: String = String::new();

    let mut operator_stack: Vec<Ops> = vec![];

    while i < main_eq.len() {
        match main_eq.chars().nth(i).unwrap() {
            ' ' => continue,
            '+' => check_precedence(&mut operator_stack, Ops::Add, &mut final_stack),
            '*' => check_precedence(&mut operator_stack, Ops::Multiply, &mut final_stack),
            '-' => check_precedence(&mut operator_stack, Ops::Subtract, &mut final_stack),
            '/' => check_precedence(&mut operator_stack, Ops::Divide, &mut final_stack),
            x => final_stack.push(x),
        }

        if main_eq.chars().nth(i).unwrap().is_numeric() {
            for iter in operator_stack.iter() {
                final_stack.push((*iter).into_char())
            }

            operator_stack.clear();
        }

        i += 1;
    }

    for iter in operator_stack.into_iter() {
        final_stack.push(iter.into_char())
    }

    println!("{:?}", eval(final_stack));
}
