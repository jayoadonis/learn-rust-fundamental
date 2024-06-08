


pub fn execute() {
    let binding = crate::model::lemon::Lemon::new("a"); //REM: binding what?
    let lemon: Box<&dyn crate::model::i_entity::IEntity> = 
        Box::new(&binding); 
    dbg!(lemon.get_id());
}