extern crate track_splitter;

#[cfg(test)]
mod tests {

    use track_splitter::split::{randomize_indices, split_percent, k_fold};
    
    #[test]
    fn check_randomized() {
        assert_ne!(randomize_indices(&mut vec![1,2,3,4,5,6], 42) , [1,2,3,4,5,6]);
    }
    #[test]
    fn check_split() {
        assert_eq!(split_percent(&mut vec![0,1,2,3,4,5,6,7,8,9], 0.7) , vec![vec![0,1,2,3,4,5,6],vec![7,8,9]]);
    }

    #[test]
    fn check_randomize_split(){
        let mut begin = vec![0,1,2,3,4,5,6,7,8,9];
        let mut rando = randomize_indices(&mut begin , 42);
        //check the indices are randomized
        assert_ne!(randomize_indices(&mut vec![0,1,2,3,4,5,6,7,8,9], 42) , vec![0,1,2,3,4,5,6,7,8,9]);
        //check these are equal when given same seed
        assert_eq!(rando, randomize_indices(&mut vec![0,1,2,3,4,5,6,7,8,9], 42));
        let rando_1 = rando[0..7].to_vec();
        let rando_2 = rando[7..].to_vec();
        //check that the splitter works
        assert_eq!( vec![rando_1,rando_2] , split_percent(&mut rando, 0.7))
    }

    #[test]
    fn check_folds(){
        let mut ten = vec![0,1,2,3,4,5,6,7,8,9];
        assert_eq!(k_fold(&mut ten, 10).len(), 10);
        let mut one_hundred : Vec<u8> = Vec::new();
        for i in 0..100{
            one_hundred.push(i);
        }
        assert_eq!(k_fold(&mut one_hundred, 10).len(), 10);
        for i in 0..10{
            assert_eq!(k_fold(&mut one_hundred, 10)[i].len(), 10);
        }
    }
}
