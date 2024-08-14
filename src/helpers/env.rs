use std::env;

pub fn get_int(env_var: &str) -> u32 {
    let string = env::var(env_var)
        .unwrap_or_else(|_| panic!("{env_var} must be set in environment as unsigned int"));
    let int = string.parse::<u32>();

    match int {
        Err(error) => panic!("Cannot parse {env_var} environment var to int! {error}"),
        Ok(int) => int,
    }
}

pub fn get_float(env_var: &str) -> f32 {
    let string = env::var(env_var)
        .unwrap_or_else(|_| panic!("{env_var} must be set in environment as float",));
    let float = string.parse::<f32>();

    match float {
        Err(error) => panic!("Cannot parse {env_var} environment var to float! {error}"),
        Ok(float) => float,
    }
}

#[cfg(test)]
mod tests {
    use super::get_float;
    use super::get_int;
    use std::env;

    #[test]
    fn test_get_int_from_env_valid() {
        env::set_var("TEST_INT_VALID", "42");
        assert_eq!(get_int("TEST_INT_VALID"), 42);
        env::remove_var("TEST_INT_VALID");
    }

    #[test]
    #[should_panic(expected = "TEST_INT_NOT_SET must be set in environment as unsigned int")]
    fn test_get_int_from_env_not_set() {
        get_int("TEST_INT_NOT_SET");
    }

    #[test]
    #[should_panic(expected = "Cannot parse TEST_INT_NOT_NUMBER environment var to int!")]
    fn test_get_int_from_env_invalid_format() {
        env::set_var("TEST_INT_NOT_NUMBER", "not_a_number");
        get_int("TEST_INT_NOT_NUMBER");
        env::remove_var("TEST_INT_NOT_NUMBER");
    }

    #[test]
    fn test_get_float_from_env_valid() {
        env::set_var("TEST_FLOAT", "3.14");
        assert_eq!(get_float("TEST_FLOAT"), 3.14);
        env::remove_var("TEST_FLOAT");
    }

    #[test]
    #[should_panic(expected = "must be set in environment as float")]
    fn test_get_float_from_env_not_set() {
        get_float("TEST_FLOAT_NEW");
    }

    #[test]
    #[should_panic(expected = "Cannot parse TEST_FLOAT_NOT_A_FLOAT environment var to float!")]
    fn test_get_float_from_env_invalid_format() {
        env::set_var("TEST_FLOAT_NOT_A_FLOAT", "not_a_float");
        get_float("TEST_FLOAT_NOT_A_FLOAT");
        env::remove_var("TEST_FLOAT_NOT_A_FLOAT");
    }
}
