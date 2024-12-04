pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn quicksort<T>(vector: Vec<T>)
where
    T: Ord,
    T: Eq,
{
}

pub fn print_vector<T>(vector: Vec<T>) {
    
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
