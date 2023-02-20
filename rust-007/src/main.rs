


fn add(a: i32, b: i32) -> i32 {
    return a+b; 
}


#[cfg(test)]
mod tests {
   


use super::*;

    #[test]
    fn test_add_ok() {
        assert_eq!(add(1, 2), 3);
    }

  

    #[test]
    fn test_add_fail() {
        assert_ne!(add(1, 2), 5);
    }



}
