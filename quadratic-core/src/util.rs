use itertools::Itertools;
use std::fmt;

/// Returns a column's name from its number.
pub fn column_name(mut n: i64) -> String {
    let negative = n < 0;
    if negative {
        n = -(n + 1);
    }

    let mut chars = vec![];
    loop {
        let i = n % 26;
        chars.push(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[i as usize]);
        n /= 26;
        if n <= 0 {
            break;
        }
        n -= 1;
    }
    if negative {
        // push byte literal of single character `n`
        chars.push(b'n');
    }

    chars.into_iter().rev().map(|c| c as char).collect()
}
/// Returns a column number from a name, or `None` if it is invalid or out of range.
pub fn column_from_name(mut s: &str) -> Option<i64> {
    let negative = s.starts_with('n');
    if let Some(rest) = s.strip_prefix('n') {
        s = rest;
    }

    fn digit(c: char) -> Option<i64> {
        ('A'..='Z').contains(&c).then(|| c as i64 - 'A' as i64)
    }

    let mut chars = s.chars();
    let mut ret = digit(chars.next()?)?;
    for char in chars {
        ret = ret
            .checked_add(1)?
            .checked_mul(26)?
            .checked_add(digit(char)?)?;
    }

    if negative {
        ret = -ret - 1;
    }

    Some(ret)
}

/// Returns a human-friendly list of things, joined at the end by the given
/// conjuction.
pub fn join_with_conjunction(conjunction: &str, items: &[impl fmt::Display]) -> String {
    match items {
        [] => format!("(none)"),
        [a] => format!("{}", a),
        [a, b] => format!("{} {} {}", a, conjunction, b),
        [all_but_last @ .., z] => {
            let mut ret = all_but_last.iter().map(|x| format!("{}, ", x)).join("");
            ret.push_str(conjunction);
            ret.push_str(&format!(" {}", z));
            ret
        }
    }
}

/// Implements `std::format::Display` for a type using arguments to `write!()`.
macro_rules! impl_display {
    ( for $typename:ty, $( $fmt_arg:expr ),+ $(,)? ) => {
        impl std::fmt::Display for $typename {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $( $fmt_arg ),+ )
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_names() {
        // Test near 0
        assert_eq!("A", column_name(0));
        assert_eq!("B", column_name(1));
        assert_eq!("C", column_name(2));
        assert_eq!("D", column_name(3));
        assert_eq!("E", column_name(4));
        assert_eq!("F", column_name(5));

        assert_eq!("nA", column_name(-1));
        assert_eq!("nB", column_name(-2));
        assert_eq!("nC", column_name(-3));
        assert_eq!("nD", column_name(-4));
        assert_eq!("nE", column_name(-5));
        assert_eq!("nF", column_name(-6));

        // Test near ±26
        assert_eq!("Y", column_name(24));
        assert_eq!("Z", column_name(25));
        assert_eq!("AA", column_name(26));
        assert_eq!("AB", column_name(27));
        assert_eq!("nY", column_name(-25));
        assert_eq!("nZ", column_name(-26));
        assert_eq!("nAA", column_name(-27));
        assert_eq!("nAB", column_name(-28));

        // Test near ±52
        assert_eq!("AY", column_name(50));
        assert_eq!("AZ", column_name(51));
        assert_eq!("BA", column_name(52));
        assert_eq!("BB", column_name(53));
        assert_eq!("nAY", column_name(-51));
        assert_eq!("nAZ", column_name(-52));
        assert_eq!("nBA", column_name(-53));
        assert_eq!("nBB", column_name(-54));

        // Test near ±702
        assert_eq!("ZY", column_name(700));
        assert_eq!("ZZ", column_name(701));
        assert_eq!("AAA", column_name(702));
        assert_eq!("AAB", column_name(703));
        assert_eq!("nZY", column_name(-701));
        assert_eq!("nZZ", column_name(-702));
        assert_eq!("nAAA", column_name(-703));
        assert_eq!("nAAB", column_name(-704));

        // Test near the integer limits
        assert_eq!("CRPXNLSKVLJFHH", column_name(i64::MAX));
        assert_eq!("nCRPXNLSKVLJFHH", column_name(i64::MIN));

        // Test fun stuff
        assert_eq!("QUADRATIC", column_name(3719092809668));
        assert_eq!("nQUADRATIC", column_name(-3719092809669));
        assert_eq!("QUICKBROWNFOX", column_name(1700658608758053877));
    }

    #[test]
    fn test_from_column_names() {
        // Test near 0
        assert_eq!(Some(0), column_from_name("A"));
        assert_eq!(Some(1), column_from_name("B"));
        assert_eq!(Some(2), column_from_name("C"));
        assert_eq!(Some(3), column_from_name("D"));
        assert_eq!(Some(4), column_from_name("E"));
        assert_eq!(Some(5), column_from_name("F"));

        assert_eq!(Some(-1), column_from_name("nA"));
        assert_eq!(Some(-2), column_from_name("nB"));
        assert_eq!(Some(-3), column_from_name("nC"));
        assert_eq!(Some(-4), column_from_name("nD"));
        assert_eq!(Some(-5), column_from_name("nE"));
        assert_eq!(Some(-6), column_from_name("nF"));

        // Test near ±26
        assert_eq!(Some(24), column_from_name("Y"));
        assert_eq!(Some(25), column_from_name("Z"));
        assert_eq!(Some(26), column_from_name("AA"));
        assert_eq!(Some(27), column_from_name("AB"));
        assert_eq!(Some(-25), column_from_name("nY"));
        assert_eq!(Some(-26), column_from_name("nZ"));
        assert_eq!(Some(-27), column_from_name("nAA"));
        assert_eq!(Some(-28), column_from_name("nAB"));

        // Test near ±52
        assert_eq!(Some(50), column_from_name("AY"));
        assert_eq!(Some(51), column_from_name("AZ"));
        assert_eq!(Some(52), column_from_name("BA"));
        assert_eq!(Some(53), column_from_name("BB"));
        assert_eq!(Some(-51), column_from_name("nAY"));
        assert_eq!(Some(-52), column_from_name("nAZ"));
        assert_eq!(Some(-53), column_from_name("nBA"));
        assert_eq!(Some(-54), column_from_name("nBB"));

        // Test near ±702
        assert_eq!(Some(700), column_from_name("ZY"));
        assert_eq!(Some(701), column_from_name("ZZ"));
        assert_eq!(Some(702), column_from_name("AAA"));
        assert_eq!(Some(703), column_from_name("AAB"));
        assert_eq!(Some(-701), column_from_name("nZY"));
        assert_eq!(Some(-702), column_from_name("nZZ"));
        assert_eq!(Some(-703), column_from_name("nAAA"));
        assert_eq!(Some(-704), column_from_name("nAAB"));

        // Test near the integer limits
        assert_eq!(Some(i64::MAX), column_from_name("CRPXNLSKVLJFHH"));
        assert_eq!(Some(i64::MIN), column_from_name("nCRPXNLSKVLJFHH"));
        assert_eq!(None, column_from_name("CRPXNLSKVLJFHI"));
        assert_eq!(None, column_from_name("XXXXXXXXXXXXXX"));
        assert_eq!(None, column_from_name("nCRPXNLSKVLJFHI"));
        assert_eq!(None, column_from_name("nXXXXXXXXXXXXXX"));

        // Test totally invalid columns
        assert_eq!(None, column_from_name("a"));
        assert_eq!(None, column_from_name("z"));
        assert_eq!(None, column_from_name("n"));
        assert_eq!(None, column_from_name("AnZ"));
        assert_eq!(None, column_from_name("nnB"));
        assert_eq!(None, column_from_name("93"));

        // Test fun stuff
        assert_eq!(Some(3719092809668), column_from_name("QUADRATIC"));
        assert_eq!(Some(1700658608758053877), column_from_name("QUICKBROWNFOX"));
    }
}
