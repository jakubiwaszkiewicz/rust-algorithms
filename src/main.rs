
mod binary_search;
mod read_line;
mod bubble_sort;

fn main() {
    //let integer_test = read_line::integer();
    //let vec_sorted_test = read_line::vec_sorted_ascending();
    let vec = read_line::_vec_unsorted();
    //println!("target: {:?}; vec_sorted: {:?}; vec: {:?}", integer_test, vec_sorted_test, vec_test)

    let sorted_vec = bubble_sort::_bubble_sort(vec);
    println!("{:?}", sorted_vec)
}
