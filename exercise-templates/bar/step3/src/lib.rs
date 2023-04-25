pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// You have to export this function as a C function

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = add(4, 5);
        assert_eq!(result, 9);
    }
}
