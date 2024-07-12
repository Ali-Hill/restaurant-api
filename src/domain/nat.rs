#[derive(Debug)]
pub struct Nat(i32);

/*
 * For this project table_no, quantity and preparation_time should be positive integers (natural numbers)
 * PostgreSQL requires i32 rather than u32 so this is ensured by this module
 */

impl Nat {
    // Returns an instance of `Item` if the input satisfies validation
    pub fn parse(n: i32) -> Result<Nat, String> {
        if n < 0 {
            Err(format!(
                "{} item is empty or contains forbidden characters.",
                n
            ))
        } else {
            Ok(Self(n))
        }
    }
}

impl AsRef<i32> for Nat {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::nat::Nat;
    use claim::{assert_err, assert_ok};

    #[test]
    fn positive_number_is_valid() {
        assert_ok!(Nat::parse(77));
    }

    #[test]
    fn zero_is_valid() {
        assert_ok!(Nat::parse(0));
    }

    #[test]
    fn negative_number_is_rejected() {
        assert_err!(Nat::parse(-1));
    }
}
