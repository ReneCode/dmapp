pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod arc;
mod datamodel;
mod line;
mod node;
mod page;

pub use arc::*;
pub use datamodel::*;
pub use line::*;
pub use node::{Node, NodeType};
pub use page::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
