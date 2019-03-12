cd


fn main() {

    let mut arr: [i32;6] = [ 5, 3, 4, 2, 1, 6];
    insertion_sort_asc(&mut arr);
    println!("{:?}", arr);
}

fn insertion_sort_asc( array: &mut [i32;6]) {    
    for i in 1..6 {
        let key = array[i];
        let mut j = i;        
        while j > 0 && array[j - 1] > key  {
            array[j] = array[j - 1];
            j = j - 1;
        }
        array[ j ] = key;
    }
}

fn insertion_sort_desc( ) {
    
    let mut array: [i32; 6] = [ 5, 3, 4, 2, 1, 6];
    println!("original:");
    println!("{:?}", array);

    println!("sorting:");
    for i in 1..6 {
        let key = array[i];
        let mut j = i;
        
        while j > 0 && array[j - 1] < key  {
            array[j] = array[j - 1];
            j = j - 1;
        }
        
        array[ j ] = key;

        println!("{:?}", array);
    }
    
    println!("result:");
    println!("{:?}", array);
}

