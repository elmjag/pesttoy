use crate::{parser::Rule, symtable::SymTable};
use pest::iterators::{Pair, Pairs};

fn unpack_one(pair: Pair<'_, Rule>) -> Pair<'_, Rule> {
    let mut parts = pair.into_inner();
    assert!(parts.len() == 1);

    parts.next().unwrap()
}

fn unpack_two(pair: Pair<'_, Rule>) -> (Pair<'_, Rule>, Pair<'_, Rule>) {
    let mut parts = pair.into_inner();
    assert!(parts.len() == 2);

    (parts.next().unwrap(), parts.next().unwrap())
}

fn as_trimed_str(pair: Pair<'_, Rule>) -> &str {
    pair.as_str().trim()
}

fn eval_atom(atom: Pair<'_, Rule>, symbols: &SymTable) -> i64 {
    assert!(atom.as_rule() == Rule::atom);

    let child = unpack_one(atom);
    match child.as_rule() {
        Rule::number => {
            let text = as_trimed_str(child);
            text.parse().unwrap()
        }
        Rule::name => {
            let var_name = as_trimed_str(child);
            symbols.get_value(var_name)
        }
        _ => panic!("unexpected rule '{:?}'", child.as_rule()),
    }
}

fn eval_unary(unary: Pair<'_, Rule>, symbols: &SymTable) -> i64 {
    assert!(unary.as_rule() == Rule::unary);

    let (op, factor) = unpack_two(unary);
    assert!(op.as_rule() == Rule::op_sub);

    -eval_factor(factor, symbols)
}

fn eval_factor(factor: Pair<'_, Rule>, symbols: &SymTable) -> i64 {
    assert!(factor.as_rule() == Rule::factor);

    let child = unpack_one(factor);
    match child.as_rule() {
        Rule::atom => eval_atom(child, symbols),
        Rule::expression => eval_expression(child, symbols),
        Rule::unary => eval_unary(child, symbols),
        _ => panic!("unexpected rule '{:?}'", child.as_rule()),
    }
}

fn eval_term(term: Pair<'_, Rule>, symbols: &SymTable) -> i64 {
    assert!(term.as_rule() == Rule::term);

    let mut parts = term.into_inner();

    let left = eval_factor(parts.next().unwrap(), symbols);

    if let Some(op) = parts.next() {
        let right = eval_factor(parts.next().unwrap(), symbols);
        match op.as_rule() {
            Rule::op_mul => left * right,
            Rule::op_div => left / right,
            _ => panic!("unexpected rule '{:?}'", op.as_rule()),
        }
    } else {
        left
    }
}

fn eval_expression(expression: Pair<'_, Rule>, symbols: &SymTable) -> i64 {
    assert!(expression.as_rule() == Rule::expression);

    let mut parts = expression.into_inner();

    let left = eval_term(parts.next().unwrap(), symbols);

    if let Some(op) = parts.next() {
        let right = eval_term(parts.next().unwrap(), symbols);
        match op.as_rule() {
            Rule::op_add => left + right,
            Rule::op_sub => left - right,
            _ => panic!("unexpected rule '{:?}'", op.as_rule()),
        }
    } else {
        left
    }
}

fn eval_assignment(assignment: Pair<'_, Rule>, symbols: &mut SymTable) {
    assert!(assignment.as_rule() == Rule::assignment);

    let mut parts = assignment.into_inner();

    let name = as_trimed_str(parts.next().unwrap());
    let value = eval_expression(parts.next().unwrap(), symbols);

    println!("{value} -> '{name}'");
    symbols.set_value(name, value);
}

fn eval_file(file: Pair<'_, Rule>, symbols: &mut SymTable) -> i64 {
    assert!(file.as_rule() == Rule::file);

    let mut parts = file.into_inner();
    let parts_len = parts.len();

    // evaluate assignmnets first
    let assignments = parts.by_ref().take(parts_len - 1);
    for pair in assignments {
        eval_assignment(pair, symbols);
    }

    // evaluate last expression
    eval_expression(parts.next().unwrap(), symbols)
}

pub fn eval(pairs: Pairs<'_, Rule>) -> i64 {
    let mut symbols = SymTable::new();

    eval_file(pairs.into_iter().next().unwrap(), &mut symbols)
}
