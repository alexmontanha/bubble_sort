fn main() {
    let arr = vec![5, 4, 3, 2, 1];
    let sorted_arr = bubble_sort(arr);
    println!("{:?}", sorted_arr);
}

fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }
    arr
}
