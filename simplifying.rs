// Challenge: https://www.codewars.com/kata/55f89832ac9a66518f000118/train/rust

fn simplify(polynomial: &str) -> String {
    let mut polynomial = String::from(polynomial);
    if polynomial.chars().nth(0) != Some('-') {
        polynomial.insert(0, '+');
    }
    
    let signs = polynomial.chars()
        .enumerate()
        .filter(|(_, c)| *c == '+' || *c == '-')
        .collect::<Vec<(usize, char)>>();
    
    let mut offset = 0;
    for i in signs {
        if !('0'..='9').any(|c| c == polynomial.chars().nth(i.0 + 1 + offset).unwrap()) {
            polynomial.insert(i.0 + 1 + offset, '1');
            offset += 1;
        }
    }
    
    let signs = polynomial.chars()
        .enumerate()
        .filter(|(_, c)| *c == '+' || *c == '-')
        .collect::<Vec<(usize, char)>>();
    
    let mut vars : Vec<(String, i32)> = vec![];
    for i in 0..signs.len() {
        let sign = signs[i];
        let coefficient_begin = sign.0 + 1;
        let coefficient_offset = polynomial[coefficient_begin..].find(|c1| !('0'..='9').any(|c2| c2 == c1));
        let mut coefficient_end = if let Some(offset) = coefficient_offset {
            offset
        } else {
            1
        };
        coefficient_end += coefficient_begin;
        let coefficient = polynomial[coefficient_begin..coefficient_end].parse::<i32>().unwrap();
        let var_begin = coefficient_end;
        let var_end = if i < signs.len() - 1 {
            signs[i + 1].0
        } else {
            polynomial.len()
        };
        let mut var_chars = polynomial[var_begin..var_end].chars().collect::<Vec<char>>();
        var_chars.sort();
        let var = var_chars.iter().collect::<String>();
        
        let modifier = if sign.1 == '-' { -1 * coefficient } else { coefficient };
        
        if let Some(pos) = vars.iter().position(|item| item.0 == var) {
            vars[pos].1 += modifier;
        } else {
            vars.push((var, modifier));
        }
    }
    
    vars.sort();
    vars.sort_by(|a, b| a.0.len().cmp(&b.0.len()));
    
    let mut simplified = String::new();
    for var in vars {
        if var.1 == 0 {
            continue;
        }
        
        let sign = if var.1 >= 0 && simplified.len() > 0 { "+" } else if var.1 < 0 { "-" } else { "" };
        let coefficient = if var.1.abs() != 1 { var.1.abs().to_string() } else { "".to_string() };
        simplified.push_str(format!("{}{}{}", sign, coefficient, var.0).as_str());
    }
    
    simplified
}
