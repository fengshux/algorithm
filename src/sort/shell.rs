use std::cmp::PartialOrd;

pub fn sort<Rhs: PartialOrd+Copy> (a: &mut [Rhs]) {

    let len = a.len();
    let mut h = 1;
    
    while  h < len/3 {
        h = 3 * h + 1;
    }

    while h >= 1 {
    
        for i in 1..len {
            let card = a[i];
            let mut j = i;
            let mut card_index = i;
            while j >= h && a[j-h]>card {
                a[j] = a[j-h];
                card_index = j - h;
                j = j - h;
            }
            if card_index != i {
                a[card_index] = card;
            }
        }
        h = h/3;
    }
    
}
