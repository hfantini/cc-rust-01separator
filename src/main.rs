fn main() {
    let mut arr: [i8; 13] = [0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1];
    println!("Input array is {:?}", arr);

    let mut i_left: usize = 0;
    let mut i_right: usize = arr.len() - 1;

    while i_left <= i_right {
        if arr[i_left] == 1 && arr[i_right] == 0 {
            let left_value = arr[i_left];
            arr[i_left] = arr[i_right];
            arr[i_right] = left_value;
            i_left += 1;
            i_right -= 1;
        } else {
            if arr[i_left] == 0 {
                i_left += 1;
            }

            if arr[i_right] == 1 {
                i_right -= 1;
            }
        }
    }
    println!("The ordered array is: {:?}", arr);
}
