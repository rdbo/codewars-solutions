// Challenge: https://www.codewars.com/kata/551f23362ff852e2ab000037/train/rust

fn climb(pyramid : &[Vec<u16>], i : usize, j : usize) -> Vec<(u16, (usize, usize))> {
    if i == 0 {
        return vec![];
    }
    
    let mut higher_nums : Vec<(u16, (usize, usize))> = vec![];
    if j < pyramid[i - 1].len() {
        higher_nums.push((pyramid[i - 1][j], (i - 1, j)));
    }
    
    if j > 0 {
        higher_nums.push((pyramid[i - 1][j - 1], (i - 1, j - 1)));
    }
    
    higher_nums
}

fn climb_top(pyramid : &[Vec<u16>], mut combs : Vec<Vec<u16>>, prev : Vec<u16>, i : usize, j : usize) -> Vec<Vec<u16>> {
    if i == 0 && j == 0 {
        combs.push(prev);
        return combs;
    }
    
    let higher_nums = climb(pyramid, i, j);
    for n in higher_nums {
        let mut p = prev.clone();
        p.push(n.0);
        let pos = n.1;
        combs = climb_top(pyramid, combs, p, pos.0, pos.1);
    }
    
    combs
}

fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    let mut sums : Vec<u16> = vec![];
    for i in 0..pyramid.last().unwrap().len() {
        let paths = climb_top(pyramid, vec![], vec![pyramid.last().unwrap()[i]], pyramid.last().unwrap().len() - 1, i);
        sums.push(
            paths.into_iter()
                .map(|v| v.iter().sum())
                .max().unwrap()
        );
    }
    
    sums.into_iter().max().unwrap()
}
