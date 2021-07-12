extern crate algorithm;

use algorithm::queue::linked::CircleQueue;


fn main() {

    let mut queue = CircleQueue::new();

    if queue.get_first().is_none() {
        println!("first is None");
    }

    for i in 'a'..='z' {
        queue.append_tail(i);
    }

    while let Some(n) = queue.get_first() {
        println!("{}",n)
    }
    
}
