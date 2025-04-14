use datamodel::DataModel;

pub trait Command: std::fmt::Debug {
    fn execute(&self, data_model: &mut DataModel);
    fn undo(&self, data_model: &mut DataModel) {}
}
