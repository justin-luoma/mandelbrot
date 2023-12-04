const POSITIONS: [[usize; 2]; 4] = [
    [0, 0],
    [0, 1],
    [1, 1],
    [1, 0],
];

pub fn i_to_xy(i: usize, size: usize) -> (usize, usize) {
    let tmp = POSITIONS[last_two_bits(i)];

    let mut i = i >> 2;

    let mut x = tmp[0];
    let mut y = tmp[1];

    let mut n = 4;
    while n <= size {
        let half = n / 2;

        match last_two_bits(i) {
            0 => {
                std::mem::swap(&mut x, &mut y);
            }
            1 => {
                y += half;
            }
            2 => {
                x += half;
                y += half;
            }
            3 => {
                let tmp = y;
                y = half - 1 - x;
                x = half - 1 - tmp;
                x += half;
            }
            _ => unreachable!()
        }

        i >>= 2;

        n *= 2;
    }

    (x, y)
}

fn last_two_bits(n: usize) -> usize {
    n & 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i_to_xy() {
        assert_eq!((0, 0), i_to_xy(0, 32));
    }
}
