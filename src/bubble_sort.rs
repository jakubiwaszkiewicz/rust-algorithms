pub (crate) fn _bubble_sort (mut vec: Vec<usize>) -> Vec<usize> {
    let lenght = vec.len();
    for i in 0..lenght {
        for j in i..lenght-1 {
            if vec[j] > vec[j+1] {
                vec.swap(j, j+1);
            }
        }
    }
    vec
}