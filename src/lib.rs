#[cfg(test)]
mod tests{
    
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{length: 8, width:7};
        let smaller = Rectangle{length: 5, width: 1};

        assert!(larger.can_hold(&smaller));
    }
}