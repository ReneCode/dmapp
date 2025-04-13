pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod command;
mod commandhandler;
mod createarccommand;
mod createlinecommand;
mod createpagecommand;

pub use commandhandler::*;
pub use createarccommand::*;
pub use createlinecommand::*;
pub use createpagecommand::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
