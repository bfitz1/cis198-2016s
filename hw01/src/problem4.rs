/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut moves = Vec::new();

    if num_discs > 0 {
        moves.append(&mut hanoi(num_discs - 1, src, dst, aux));
        moves.push((src, dst));
        moves.append(&mut hanoi(num_discs - 1, aux, src, dst));
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hanoi_with_one_disk() {
        let result = hanoi(1, Peg::A, Peg::B, Peg::C);
        assert_eq!(vec![(Peg::A, Peg::C)], result);
        assert_eq!(1, result.len());
    }
}