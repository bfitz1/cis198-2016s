/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut indices = {
        let mut vs = vec![false, false];
        for _ in 0..n-2 {
            vs.push(true);
        }
        vs
    };

    let mut primes = Vec::new();
    for i in 0..indices.len() {
        if indices[i] == true {
            primes.push(i as u32);
            for j in 2.. {
                if i * j < indices.len() {
                    indices[i * j] = false;
                } else {
                    break;
                }
            }
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sieve() {
        assert_eq!(vec![2, 3, 5, 7, 11], sieve(12));
    }
}