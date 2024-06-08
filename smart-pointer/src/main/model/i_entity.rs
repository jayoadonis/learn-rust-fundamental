


pub trait IEntity<'a> {
    fn get_id(&self) -> Result<&'a str, crate::model::entity_error::EntityError<'a>>;
}