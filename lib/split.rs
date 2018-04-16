 use rand::{Rng, SeedableRng, StdRng};

/// Randomizes the indices of a given mutable vector and returns it as an array
///
/// # Arguments
/// 
/// * indices : &mut Vec<T> - a mutable vector of generics 
/// 
/// * seed_val : usize - a usize which will be used as the seed
/// 
/// # Returns 
/// 
/// * vector of generics randomized by rust's Seedable Standard RNG
pub fn randomize_indices<T>(indicies: &mut Vec<T>, seed_val: usize)->Vec<T> where T: Clone{
    
    let seed: &[_] = &vec![seed_val];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    rng.shuffle(indicies);
    return indicies.to_vec()
}

/// Takes in a vector of generics and a value between 0 and 1  
///  and returns two slices
/// 
/// # Arguments
/// 
/// * randomized_data : vector of generics - assumed pre-randomized
/// 
/// * percent_training : f64 - the decimal value we will use to determine where
///         the split in the training set will be
/// 
/// # Notes
/// 
/// The floor of the length of the given array multiplied by the training split
///  percent will be used as the split index
/// 
/// # Returns 
/// * two vectors within a vector 
///     * s[0] is the training set
///     * s[1] is the test set
pub fn split_percent<T>(randomized_data: &mut Vec<T>, percent_training: f64)->Vec<Vec<T>> where T: Clone{
    
    let split_index : usize = ((randomized_data.len() as f64) * percent_training).floor() as usize;
    let trainers = &randomized_data[0..split_index];
    let testers  = &randomized_data[split_index..];
    let mut trainer_cases : Vec<T> = Vec::new();
    let mut tester_cases: Vec<T> = Vec::new();

    for i in trainers.iter(){
        trainer_cases.push(i.clone());
    }
    for i in testers.iter(){
        tester_cases.push(i.clone());
    }
    return vec![trainer_cases, tester_cases];
}

/// Takes in a vector of generics and a value (k) between 0 and 255
///  and returns k number of slices
/// 
/// # Arguments
/// 
/// * randomized_data : vector of generics - assumed pre-randomized
/// 
/// * k : u8 - the integer number of folds on the data set (traditionally 10)
/// 
/// # Notes
/// 
/// Should there be extra terms, these will appear in the final fold. 
/// 
/// # Returns 
/// * One super vector of the k folds 
///     * s[i] is the ith fold of k total folds 
/// 
pub fn k_fold<T>(randomized_data: &mut Vec<T>, k: u8)->Vec<Vec<T>> where T: Clone{
    let fold_size : u32 = (randomized_data.len() as f64/ k as f64).floor() as u32;
    let mut folds : Vec<Vec<T>> = Vec::new();
    for i in 0..k{
        let multi = (i*fold_size) as usize;
        let multi_two = ((i+1)*fold_size) as usize;
        if i < (k-1){    
            folds.push(randomized_data[(multi)..(multi_two)].to_vec());
        }
        else{
            folds.push(randomized_data[(multi)..].to_vec());
        }
    }
    return folds;
}