
#[derive(Debug)]
pub struct Lemon<'a> {
    id: &'a str
}

impl<'a> Lemon<'a> {
    pub fn new( id: &'a str) -> Self {
        return Self {
            id
        };
    }
}

impl<'a> crate::model::i_entity::IEntity<'a> for Lemon<'a> {
    //REM: @Override
    fn get_id(&self) -> 
        Result<&'a str, crate::model::entity_error::EntityError<'a>> 
    {
        if( self.id.trim().is_empty() ) {
            return Err(
                crate::model::entity_error::EntityError::new(
                    0x1,
                    "Invalid Id."
                )
            );
        }
        return Ok(self.id);
    }
}