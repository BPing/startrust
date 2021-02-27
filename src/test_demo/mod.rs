/// 测试函数描述文档
///
/// # Example
///
/// ```
/// use generics_demo::moda;
///
/// let sum:i64=moda::add_func(2,2);
///
/// ```
///
///
/// Notice :
pub fn add_func(a: i64, b: i64) -> i64 {
    internal_adder(a, b)
}

fn internal_adder(a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_func() {
        assert_eq!(add_func(2, 2), 4, "test_add_func fail");
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }


    #[test]
    #[ignore]
    fn ignore() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        panic!("Make this tests fail");
    }

    #[test]
    #[should_panic(expected = "Make this tests fail")]
    fn should_panic_expected() {
        panic!("Make this tests fail");
    }

    #[test]
    #[should_panic(expected = "Make this tests fail")]
    fn should_panic_expected_fail() {
        panic!("success");
    }

    #[test]
    fn result_ok() -> Result<(), String> {
        if add_func(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn result_fail() -> Result<(), String> {
        if add_func(2, 2) == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}