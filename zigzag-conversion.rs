impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
      
        let chars: Vec<char> = s.chars().collect();
        let mut x = 0;
        let mut y = 0;
        let mut descend = true;
        let mut rows: Vec<Vec<Option<char>>> = 
            vec![vec![None; s.len() as usize]; num_rows as usize];
        
        // goes down to last row (y+1)
        // then upward + rightward (x+1, y-1)
        // repeat
        for c in chars {
            rows[y][x] = Some(c);
            if descend {
                y += 1;
                if y >= num_rows as usize {
                    y -= 2;
                    x += 1;
                    descend = false;
                }
            } else if y <= 0 {
                y += 1;
                descend = true;
            } else {
                x += 1;
                y -= 1;
            }
        }

        let mut solution: String = String::new();
        for r in rows {
            for c in r {
                match c {
                    Some(c) => {
                        solution.push(c);
                    },
                    None => {}
                }
            }
        }

        solution
    }
}
