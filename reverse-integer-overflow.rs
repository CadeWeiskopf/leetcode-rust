impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let scalar = if x > 0 { 1 } else { -1 };
        let mut original: i32 = x.abs();
        let mut reversed: i32 = 0;
        while original > 0 { 
            let r: (i32, bool) = reversed.overflowing_mul(10);
            if r.1 {
                return 0;
            }
            let result: (i32, bool) = r.0.overflowing_add(original % 10);
            if result.1 {
                return 0;
            }
            reversed = result.0;
            original /= 10;
        }
        return reversed * scalar;
    }
}
