extern crate algorithm;
extern crate rand;
use algorithm::sort::selection;
use algorithm::sort::insertion;
use algorithm::sort::shell;
use algorithm::watch::timer::Timer;
// use std::vec::Vec;
use rand::Rng;

fn main() {

    let N = 1000;
    let T = 100;
    let MAX = N * N;    
    let mut rng = rand::thread_rng();

    let mut insertion_t = 0;
    let mut selection_t = 0;
    let mut shell_t = 0;
    
    for _ in (0..T) {
        let mut vec = Vec::new();
        for i in (0..N) {            
            vec.push(rng.gen_range(0, MAX));
        }
        let mut v2 = vec.clone();
        let mut v3 = vec.clone();
        
        let t1 = Timer::new();        
        insertion::sort(&mut vec);        
        insertion_t += t1.end();

        
        let t2 = Timer::new();
        selection::sort(&mut v2);
        selection_t += t2.end();

                
        let t3 = Timer::new();
        shell::sort(&mut v3);
        shell_t += t3.end();

    }
    println!("insertion is {} m", insertion_t);
    println!("selection is {} m", selection_t);
    println!("shell is {} m", shell_t);
    
}
