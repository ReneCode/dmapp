pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod arccommand;
mod command;
mod commandhandler;
mod commandline;
mod exportcommand;
mod linecommand;
mod pagecommand;

pub use arccommand::*;
pub use commandhandler::*;
pub use commandline::*;
pub use exportcommand::*;
pub use linecommand::*;
pub use pagecommand::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
