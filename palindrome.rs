impl Solution {
    pub fn is_palindrome_i32(x: i32) -> bool {
        if (x < 0) {
            return false;
        }

        let mut original: i32 = x;
        let mut reversed: i32 = 0;
        while original > 0 {
            reversed = (reversed * 10) + (original % 10);
            original /= 10;
        }

        return x == reversed;
    }
    
    pub fn is_palindrome_str(x: i32) -> bool {
        let xs = x.to_string();
        let xsIt = xs.chars();
        let xsRevIt = xs.chars().rev();
        let mut k = 0;
        let max = xs.len() / 2;
        for (i, j) in xsIt.zip(xsRevIt) {
            if (i != j) {
                return false;
            }
            k += 1;
            if (k >= max) {
                break;
            }
        }
        return true;
    }
}
