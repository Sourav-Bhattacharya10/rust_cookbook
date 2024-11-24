use std::thread;

pub fn find_max(arr: &[i32]) -> Option<i32> {
    const MIN_NUM_IN_ARRAY: usize = 2;

    if arr.len() <= MIN_NUM_IN_ARRAY {
        return arr.iter().cloned().max();
    }

    let mid: usize;
    if arr.len() % 2 == 0 {
        mid = arr.len() / 2;
    } else {
        mid = (arr.len() / 2) + 1;
    }

    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let handle_left = thread::spawn(move || find_max(&left));
    let handle_right = thread::spawn(move || find_max(&right));

    let max_left = handle_left.join().unwrap()?;
    let max_right = handle_right.join().unwrap()?;

    Some(max_left.max(max_right))
}
