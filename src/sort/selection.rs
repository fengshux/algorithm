
// struct Selection {}

// impl Selection {
//     fn sort(a: &mut [PartialEq]) {

//         let len = a.len();
//         for i in 0..len {
//             let min_index = i;
//             for j in (i+1)..len {
//                 if a[j] < a[min_index] {
//                     min_index = j
//                 }
//             }
//             if min_index != i {
//                 let temp = a[i];
//                 a[i] = a[min_index];
//                 a[min_index] = temp;
//             }            
//         }        
//     }
// }
use std::cmp::PartialOrd;

pub fn sort<Rhs: PartialOrd+Copy> (a: &mut [Rhs]) {

    let len = a.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i+1)..len {
            if a[j] < a[min_index] {
                min_index = j
            }
        }
        if min_index != i {
            let temp = a[i];
            a[i] = a[min_index];
            a[min_index] = temp;
        }            
    }        
}
