pub fn generate_prime_number(limit: i64) -> Option<i64> {
    if limit < 2 {
        return None;
    }

    let limit = limit as usize;
    let mut is_prime = vec![true; limit + 1];
    let mut p = 2;

    while p * p <= limit {
        if is_prime[p] {
            for i in (p * p..=limit).step_by(p) {
                is_prime[i] = false;
            }
        }
        p += 1;
    }

    for num in (2..=limit).rev() {
        if is_prime[num] {
            return Some(num as i64);
        }
    }

    None
}

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = gcd_extended(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (gcd, x, y)
    }
}
