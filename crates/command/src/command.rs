use datamodel::DataModel;

pub trait Command: std::fmt::Debug {
    fn execute(&self, dm: &mut DataModel);
    fn undo(&self, dm: &mut DataModel);
}
