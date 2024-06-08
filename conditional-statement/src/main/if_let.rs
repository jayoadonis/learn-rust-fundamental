


pub fn execute() -> Option<bool> {
    let x: Result<bool, usize> = Ok::<bool, usize>(false);
    if let Ok(x1) = x {
        return Some(x1);
    }
    dbg!(
        format!("Is an Error occured: {}", x.is_err()) 
    );
    return None;
}