

pub fn execute() -> Option<bool> {
    let x: Result<bool, usize> = Ok::<bool, usize>(true);
    let result: Option<bool> = match x {
        Ok(x1) => { return Some(x1) },
        Err(x1) => { 
            dbg!(
                format!("Is an Error occured: {}, code = {}", x.is_err(), x1) 
            );
            return None;
        }
    };
    return result;
}