fn bit_len(n: usize) -> usize {
    let mut len = 0;
    loop {
        if (1 << len) > n {
            break;
        }
        len += 1;
    }

    len
}

fn bit_sum(n: usize) -> usize {
    let mut ret = 0;
    let mut bit = 1;
    while bit <= n {
        if n & bit > 0 {
            ret += 1;
        }
        bit <<= 1;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_sum_test() {
        assert_eq!(0, bit_sum(0));
        assert_eq!(1, bit_sum(1));
        assert_eq!(1, bit_sum(1));
        assert_eq!(1, bit_sum(2));
        assert_eq!(2, bit_sum(3));
        assert_eq!(3, bit_sum(7));
    }

    #[test]
    fn bit_len_test() {
        assert_eq!(0, bit_len(0));
        assert_eq!(1, bit_len(1));
        assert_eq!(2, bit_len(2));
        assert_eq!(2, bit_len(3));
    }
}
