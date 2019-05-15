/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for n in slice {
        sum += *n;
    }
    sum
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut new_vs = Vec::new();
    for item in vs.iter() {
        if !new_vs.contains(item) {
            new_vs.push(*item);
        }
    }
    new_vs
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut new_vs = Vec::new();
    for &item in vs.iter() {
        if pred(item) {
            new_vs.push(item)
        }
    }
    new_vs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sum() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(sum(&array), 15);
    }

    #[test]
    fn basic_dedup() {
        let vs = vec![1, 2, 2, 3, 4, 1];
        assert_eq!(dedup(&vs), vec![1, 2, 3, 4]);
    }

    #[test]
    fn basic_filter() {
        fn even_predicate(x: i32) -> bool {
            (x % 2) == 0
        }

        let vs = vec![1, 2, 3, 4, 5];
        assert_eq!(filter(&vs, &even_predicate), vec![2, 4]);
    }
}