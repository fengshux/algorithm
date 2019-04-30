use std::cmp::PartialOrd;

pub fn sort<Rhs: PartialOrd+Copy> (a: &mut [Rhs]) {

    let len = a.len();
    for i in 1..len {
        let card = a[i];
        let mut j = i;
        let mut card_index = i;
        while j >= 1 && a[j-1]>card {
            a[j] = a[j-1];
            card_index = j - 1;
            j = j - 1;
        }
        if card_index != i {
            a[card_index] = card;
        }
    }        
}
