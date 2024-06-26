pub mod fifth;
pub mod first;
pub mod fourth;
pub mod second;
pub mod silly1;
pub mod silly2;
pub mod sixth;
pub mod third;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
