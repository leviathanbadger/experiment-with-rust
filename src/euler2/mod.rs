mod tests;

fn fib(mut n: i32) -> i32 {
    if n < 2 {
        1
    } else if n == 2 {
        2
    } else {
        let mut a = 1;
        let mut b = 1;
        while n > 1 {
            let tmp = a + b;
            a = b;
            b = tmp;
            n -= 1;
        }
        b
    }
}

fn sum_even_valued_fib(up_to: i32) -> i32 {
    let mut sum = 0;
    let mut idx = 0;
    loop {
        let val = fib(idx);
        if val > up_to { break; }
        if val % 2 == 0 { sum += val; }
        idx += 1;
    }
    sum
}

#[allow(dead_code)]
pub fn euler2() -> i32 {
    let result = sum_even_valued_fib(4_000_000);

    result
}
