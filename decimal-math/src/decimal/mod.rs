// decimal/mod.rs
//mod math;           // includes math.rs

// pub use math::*;    // re-exports everything from math.rs publicly
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;
//use std::ops::AddAssign;
use std::fmt;

#[derive(Clone,Copy)]
pub struct Decimal {
    pub digits: [u8; 32],
    pub exponent: i32,
    pub negative: bool,
}

impl Neg for Decimal {
    type Output = Decimal;
    fn neg(self) -> Decimal {
        Decimal{
            digits: self.digits,
            exponent: self.exponent,
            negative: !(self.negative)
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Decimal) -> bool{
        self.digits == other.digits && self.exponent == other.exponent && self.negative == other.negative
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

         if self.negative != other.negative {
            return if self.negative { 
                Some(Ordering::Less) 
            } else { 
                Some(Ordering::Greater) 
            };
        }
        
        if self.exponent != other.exponent {
            let cmp = self.exponent.cmp(&other.exponent);
            return if self.negative { 
                Some(cmp.reverse())
            } else { 
                Some(cmp) 
            };
        }
        
        let cmp = self.digits.cmp(&other.digits);
        return if self.negative {
            Some(cmp.reverse())
        } else {
            Some(cmp)
        };
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let last_nonzero = self.digits.iter()
        .rposition(|&d| d != 0);
        if self.negative {
            s.push('-');
        }
        match last_nonzero {
            None => {
                s = "0".to_string();
            }
            Some(i) => {
                s.push((self.digits[0] + b'0') as char);
                for n in 1..(i+1) {
                    if n == 1 {
                        s.push('.');
                    }
                    s.push((self.digits[n] + b'0') as char);
                    
                }
            }
        }
        s.push_str(" * 10^");
        s.push_str(&self.exponent.to_string());
        write!(f, "{}", s)
    }
}

impl Decimal {
    pub fn from_str(n: &str) -> Self {
        let mut exponent = 0i32;
        let mut digits = [0u8; 32];
        let mut shift = 0usize;
        let mut negative = false;
        let mut leading = true;
        let mut decimal = false;
        for (i, c) in n.chars().enumerate() {
            if c != '0' && c != '.' && c != '-' && leading {
                exponent -= 1;
                if i != 0 {
                    shift += 1;
                }
                leading = false;
            } else if leading && decimal {
                exponent -= 1;
                shift += 1;
            }
            if !decimal && !leading {
                exponent+=1;
            }
            if i - shift == 32 {
                if c as u8 - b'0' >= 5 {
                    digits[31] += 1;
                    exponent += (n.chars().count() - i - 1) as i32;
                }
                break;
            }
            if c == '-' && i == 0 {
                negative = true;
                shift += 1;
            } else if c == '.' {
                exponent = ((i - shift) - 1) as i32; 
                shift += 1;
                decimal = true;
            } else if !leading {
                digits[i - shift] = c as u8 - b'0';
            }
        }
        for i in 0..32 {
            let m = 31 - i;
            if digits[m] >= 10 {
                digits[m-1] += digits[m] / 10;
                digits[m] %= 10;
            }
        }
        Decimal{
            digits,
            exponent,
            negative
        }
    }
    pub fn from_i32(n: i32) -> Self {
        let mut m = n;
        let mut i = 0;
        let mut digits = [0u8; 32];
        let mut exp = -1i32;
        let mut leading = true;
        let mut digits_temp = [0u8; 10];
        let mut first_digit = 0i32;
        let mut last_digit = 0i32;
        while m > 0 {
            exp += 1;
            digits_temp[i] = (m % 10) as u8;
            if digits_temp[i] != 0 {
                if leading {
                    first_digit = i as i32;
                }
                last_digit = i as i32;
                leading = false;
            }
            m /= 10;
            i += 1;
        }
        i = 0;
        /*if last_digit == first_digit {
            exp = 0;
        }*/
        if leading == true {
            exp = 0;
        }
        while last_digit >= first_digit {
            digits[i] = digits_temp[last_digit as usize];
            last_digit-=1;
            i += 1;
        }
        Decimal{
            digits: digits,
            exponent: exp,
            negative: false
        }
    }
    pub fn normalize(self) -> Decimal {
        let mut z = self;
        let last_nonzero = z.digits.iter()
        .rposition(|&d| d != 0);
        match last_nonzero {
            None => {
                z.exponent = 0;
            }
            Some(i) => {
                z.exponent = i as i32;
            }
        }
        z
    }
    pub fn fix_leading(self) -> Decimal {
        let mut z = self;
        let mut y = [0u8; 32];
        let mut leading = true;
        let mut i = 0;
        let mut all_zeroes = true;
        for digit in z.digits {
            if leading && digit != 0 {
                leading = false;
                all_zeroes = false;
            }
            if leading {
                z.exponent -= 1;
            }
            if !leading {
                y[i] = digit;
                i+=1;
            }
        }
        if all_zeroes {
            z.exponent = 0;
        }
        z.digits = y;
        z
    }
    pub fn new() -> Self {
        Decimal{
            digits: [0u8; 32],
            exponent: 0i32,
            negative: false
        }
    }
    pub fn pow(self, mut other: i32) -> Decimal {
        let mut new = self;
        let mut neg_pow = false;
        let one = Decimal::from_i32(1i32);
        if other == 0 {
            return one;
        }
        if other < 0 {
            other = -other;
            neg_pow = true
        }
        for _ in 0..(other-1) {
            new = new * self;
        }
        if neg_pow {
            return one / new;
        }
        new
    }
}

impl Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal {
        if self.negative && other.negative {
            return - (self + other);
        } else if self.negative {
            let mut s = self;
            s.negative = false;
            return other - s;
        } else if other.negative {
            let mut o = other;
            o.negative = false;
            return self - o;
        }
        let bigger = if self >= other { self } else { other };
        let smaller = if self < other { self } else { other };
        let diff = (bigger.exponent - smaller.exponent) as usize;

        // buf[0] is an overflow slot above bigger's leading digit;
        // buf[1..=32] line up with bigger.digits[0..32]. u16 so the
        // per-column sums can't overflow before we propagate carries.
        let mut buf = [0u16; 33];
        for k in 0..32 {
            buf[k + 1] = bigger.digits[k] as u16;
        }
        for j in 0..32 {
            let k = j + diff; // bigger-index that smaller.digits[j] aligns to
            if k < 32 {
                buf[k + 1] += smaller.digits[j] as u16;
            }
        }
        // round in the smaller digit that falls off the right edge
        if diff >= 1 && diff <= 32 && smaller.digits[32 - diff] >= 5 {
            buf[32] += 1;
        }

        // single carry-propagation pass, least- to most-significant
        for k in (1..=32).rev() {
            if buf[k] >= 10 {
                buf[k - 1] += buf[k] / 10;
                buf[k] %= 10;
            }
        }

        let mut digits = [0u8; 32];
        let exponent;
        if buf[0] > 0 {
            // a carry overflowed the leading digit: shift right by one
            exponent = bigger.exponent + 1;
            digits[0] = buf[0] as u8;
            for k in 1..32 {
                digits[k] = buf[k] as u8;
            }
            if buf[32] >= 5 {
                let mut l = 31;
                while digits[l] == 9 {
                    digits[l] = 0;
                    if l == 0 { break; }
                    l -= 1;
                }
                digits[l] += 1;
            }
        } else {
            exponent = bigger.exponent;
            for k in 0..32 {
                digits[k] = buf[k + 1] as u8;
            }
        }

        let all_zeroes = digits.iter().all(|&d| d == 0);
        Decimal{
            digits,
            exponent: if all_zeroes { 0 } else { exponent },
            negative: false
        }
    }
}

impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, other: Decimal) -> Decimal {
        if other.negative && self.negative {
            return other + self;
        } else if other.negative {
            let mut o = other;
            o.negative = false;
            return self + o;
        } else if self.negative {
            let mut s = self;
            s.negative = false;
            return - (s + other);
        }
        if other > self {
            return -(other - self);
        }
        let s = self.fix_leading();
        let o = other.fix_leading();
        let bigger = if s >= o { s } else { o };
        let smaller = if s >= o { o } else { s };
        let mut digits = bigger.digits;
        let mut local_exp = bigger.exponent;
        let mut exponent = bigger.exponent;
        for i in 0..32usize {
            if local_exp <= smaller.exponent && i as i32 >= (bigger.exponent - smaller.exponent) {
                //println!("{} {}", bigger, smaller);
                let small = smaller.digits[i - (bigger.exponent - smaller.exponent) as usize];
                while digits[i] < small {
                    let mut l = i - 1;
                    while digits[l] == 0 {
                        digits[l] = 9;  // a borrowed 10 fills this zero with 9
                        l -= 1;
                    }
                    digits[l] -= 1;
                    digits[i] += 10;  
                }
                digits[i] -= small;
            }
            local_exp -= 1;
        }
        if local_exp < bigger.exponent - 31 && 32 - (bigger.exponent - smaller.exponent) <= 31 {
            if smaller.digits[32 - (bigger.exponent - smaller.exponent) as usize] >= 5 {
                let mut l = 31;
                while digits[l] == 0 {
                    digits[l] = 9;  // a borrowed 10 fills this zero with 9
                    l -= 1;
                }
                digits[31] -= 1;
        }
        }
        let mut all_zeroes = true;
        for n in 0..32 { 
            let i = 31 - n;
            if digits[i] != 0 {
                all_zeroes = false;
            }
            while digits[i] >= 10 {
                digits[i - 1] += 1;
                digits[i] -= 10;
            }
        }
        if all_zeroes {
            exponent = 0;
        }
        let new = Decimal{
            digits: digits,
            exponent: exponent,
            negative: false
        }.fix_leading();
        new
    }
}

impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, other: Decimal) -> Decimal {
        let big = if self >= other {self} else {other};
        let small = if self >= other {other} else {self};
        let mut new = Decimal::new();

        let max_exponent = big.exponent + small.exponent + if big.digits[0] * small.digits[0] >= 10 {1} else {0};
        let min_exponent = max_exponent - 31;
        let five = Decimal::from_i32(5);

        let one = Decimal::from_i32(1i32);

        for i in 0..32 {
            let m = 31 - i;
            for p in 0..32 {
                let result = big.digits[p] * small.digits[m];
                let mut exp = big.exponent - p as i32 + small.exponent - m as i32 + if result >= 10 {1} else {0};
                    
                if exp >= min_exponent - 1 {
                    let mut res_dec = Decimal::from_i32(result as i32);
                    if exp == min_exponent - 1 {
                        if res_dec >= five {
                            res_dec = one;
                            exp = min_exponent;
                        }
                    }
                    res_dec.exponent = exp;
                    
                    new = new + res_dec;
                } 
                //println!("{}", new);
            }
        }

        new
    }
}

impl Div for Decimal {
    type Output = Decimal;
    fn div(self, other: Decimal) -> Decimal {
        let other_s = other;
        let s = self.normalize();
        let mut i = 0usize;
        let mut m = 0usize;
        let mut quotient = Decimal::new();
        let mut exponent = self.exponent - other_s.exponent;
        let mut new = Decimal::new();
        let mut leading = true;
        let zero = Decimal::from_i32(0i32);
        let five = Decimal::from_i32(5);
        while m <= 32 {
            let mut p = 0;
            new = new.fix_leading();
            
            if new == zero {
                if i > 31 {
                    break;
                }
                new = Decimal::from_i32(s.digits[i] as i32);
            } else {
                new.exponent += 1;
                if i < 32 {
                    new = new + Decimal::from_i32(s.digits[i] as i32);
                }
            }
            if m == 32 {
                if new >= five {
                    quotient.digits[31] += 1;
                }
                break;
            }
            if new < other {
                if m == 32 {
                    break;
                }
                if !leading {
                    quotient.digits[m] = 0;
                    m += 1;
                    i+=1;
                    continue;
                }
            }
            while new >= other {
                new = new - other;
                p += 1;
                
            }
            if m > 31 {
                break;
            }
            if p >= 10 {
                if m == 0 {
                    exponent += 1;
                    quotient.digits[m] = p / 10;
                    p %= 10;
                    m += 1;
                } else {
                    quotient.digits[m - 1] += p / 10;
                    p %= 10;
                }
            }
            quotient.digits[m] = p;
            if leading && p != 0 {
                leading = false;
            } 
            m += 1;
            i += 1;
        }
        for i in (1..32).rev() {
            if quotient.digits[i] >= 10 {
                quotient.digits[i-1] += quotient.digits[i] / 10;
                quotient.digits[i] %= 10;
            }
        }
        quotient.exponent = exponent;
        quotient.negative = self.negative ^ other.negative;
        quotient = quotient.fix_leading();
        let mut s = self;
        s.exponent = other.exponent;
        s.negative = false;
        if s >= other && quotient.exponent < self.exponent - other.exponent {
            quotient.exponent = self.exponent - other.exponent;
        }
        quotient
        
    }
}