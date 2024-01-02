impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut vec: Vec<Vec<bool>> = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        let (mut ansIndex1, mut ansIndex2): (i32, i32) = (0,0);

        // 1 char palindromes
        for i in 0..s.len() {
            let mut temp: Vec<bool> = Vec::new();
            for j in 0..s.len() {
                if j == i {
                    temp.push(true);
                } else {
                    temp.push(false);
                }
            }
            vec.push(temp);
        }

        // 2 char palindromes
        for (i, c) in chars.iter().enumerate() {
            if i + 1 < vec[i].len() {
                vec[i][i + 1] = (*c == chars[i + 1]);
                if (vec[i][i + 1]) {
                    ansIndex1 = i as i32;
                    ansIndex2 = (i + 1) as i32;
                }
            }
        }

        for k in 2..s.len() {
            for i in 0..s.len() - k {
                let j = i + k;
                vec[i][j] = chars[i] == chars[j] && vec[i + 1][j - 1] == true;
                if (vec[i][j]) {
                    ansIndex1 = i as i32;
                    ansIndex2 = j as i32;
                }
            }
        }
        
        let mut solution: String = "".to_string();
        for (i, c) in chars.iter().enumerate() {
            if i as i32 >= ansIndex1 && i as i32 <= ansIndex2 {
                solution.push(*c);
            }
        }

        return solution;
    }
}
