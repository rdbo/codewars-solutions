// Challenge: https://www.codewars.com/kata/540afbe2dc9f615d5e000425/train/rust

struct Sudoku{
    data: Vec<Vec<u32>>,
}

impl Sudoku{
    fn is_valid(&self) -> bool {
        println!("data: {:?}", self.data);
        let n = self.data.len();
        let mut nums : Vec<u32> = vec![];
        
        // check columns and rows
        for row in &self.data {
            if row.len() != n {
                return false;
            }
            
            let val = row[0];
            
            if val < 1 || val > n as u32 || nums.iter().any(|&v| v == val) {
                return false;
            }
            
            nums.push(val);
            
            for i in 0..row.len() {
                for j in (i + 1)..row.len() {
                    if row[j] == row[i] {
                        return false;
                    }
                }
            }
        }
        
        // check squares
        let square_width = (n as f64).sqrt();
        if square_width % 1.0 != 0.0 {
            return false;
        }
        let square_width = square_width as usize;
        
        for i in 0..square_width {
            nums = vec![];
            for j in 0..square_width {
                let square = &self.data[i * square_width..(i + 1) * square_width]
                    .iter()
                    .map(|v| v[j * square_width..(j + 1) * square_width].to_owned())
                    .flatten()
                    .collect::<Vec<u32>>();
                
                for k in 0..square.len() {
                    for l in (k + 1)..square.len() {
                        if square[l] == square[k] {
                            return false;
                        }
                    }
                }
            }
        }
        
        true
    }
}
