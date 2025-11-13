// Success but weird
fn env<T>((name, expected): (&str, &str)) -> Option<T>
where
    T: FromStr,
{
    let val = match extracted_function(name, expected) {
        Ok(value) => value,
        Err(value) => return value,
    };
    Some(
        val.parse()
            .unwrap_or_else(|_| parse_failure(name, expected)),
    )
}

fn extracted_function(name: &str, expected: &str) -> Result<_, _> {
    let val = match std::env::var(name) {
        Ok(val) => val,
        Err(VarError::NotPresent) => return Err(None),
        Err(VarError::NotUnicode(_)) => parse_failure(name, expected),
    };
    Ok(val)
}


fn env<T>((name, expected): (&str, &str)) -> Option<T>
where
    T: FromStr,
{
    let val = match std::env::var(name) {
        Ok(val) => val,
        Err(VarError::NotPresent) => return None,
        Err(VarError::NotUnicode(_)) => parse_failure(name, expected),
    };
    Some(
        val.parse()
            .unwrap_or_else(|_| parse_failure(name, expected)),
    )
}

// Fail on the generic signature

/// Return an array holding `N` random bytes.
pub fn get_random_bytes<const N: usize>() -> [u8; N] {
    use ring::rand::{SecureRandom, SystemRandom};

    let mut array = [0; N];
    SystemRandom::new().fill(&mut array).expect("Error generating random values");

    array
}

pub fn get_random_bytes<const N: usize>() -> [u8; N] {
    extracted_function()
}

fn extracted_function() -> [u8; N] {
    use ring::rand::{SecureRandom, SystemRandom};

    let mut array = [0; N];
    SystemRandom::new().fill(&mut array).expect("Error generating random values");

    array
}