use rand::{thread_rng, Rng};

pub fn perm(n: u32) -> Vec<u32> {
    let n = n  as usize;
    let mut rng = thread_rng();
    let v: Vec<u32> = vec![0; n];
    
    
    for i in 0..n {
        let j: u32  = rng.gen_range(0);
        v[i] = v[j];
        v[j] = i as u32;
    }    
    return v;
}



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_perm()  {        
        let v = perm(5);
        assert_eq!(v.len(), 5);
    }

}
