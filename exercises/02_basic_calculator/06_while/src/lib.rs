// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    let mut curr_total = 0;
    let mut next_int = n;

    if n == 0 || n == 1 {
        return 1;
    }
    
    while next_int > 0 {
        if curr_total == 0 {
            curr_total = next_int * (next_int - 1);
            continue;
        }
        
        curr_total = curr_total * next_int;
        next_int = next_int - 1
    }

    curr_total
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
