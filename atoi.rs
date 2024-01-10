impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sol: i32 = 0;
        let mut is_neg: bool = false;
        let mut reading: bool = false;
        let mut dec: u32 = 0;
        let sb = s.as_bytes();
        for i in 0..s.len() {
            match sb[i] {
                32_u8 => {
                    if reading {
                        break;
                    }
                },
                43_u8 => {
                    if reading {
                        break;
                    }
                    reading = true;
                },
                45_u8 => {
                    if reading {
                        break;
                    }
                    is_neg = true;
                    reading = true;
                },
                46_u8 => {
                    dec += 1;
                },
                48_u8..=57_u8 => {
                    reading = true;
                    let sbi = (sb[i] - b'0') as i32;
                    if dec > 0 {
                        sol = sol + sbi / (i32::pow(10, dec));
                        if let Some(val) = (10 as i32).checked_pow(dec).and_then(|v| sbi.checked_div(v)).and_then(|v| sol.checked_add(v)) {
                            sol = val;
                        } else {
                            return if is_neg { i32::MIN } else { i32::MAX };
                        }
                    } else {
                        if let Some(val) = sol.checked_mul(10).and_then(|v| v.checked_add(sbi)) {
                            sol = val;   
                        } else {
                            return if is_neg { i32::MIN } else { i32::MAX };
                        }
                    }
                }
                _ => {
                    if sol == 0 || reading {
                        break;
                    }
                }
            }
        }
        if is_neg {
            sol = sol * -1;
        }
        sol
    }
}
