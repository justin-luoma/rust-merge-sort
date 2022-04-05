pub fn merge_sort<T: Copy + PartialOrd>(arr: Vec<T>) -> Vec<T> {
    let len = arr.len();
    let mut sorted = arr;
    sort(&mut sorted, 0, len - 1);

    sorted
}

pub fn merge_sort_inplace<T: Copy + PartialOrd>(arr: &mut Vec<T>) {
    let len = arr.len();
    sort(arr, 0, len - 1);
}

fn sort<T: Copy + PartialOrd>(arr: &mut Vec<T>, left: usize, right: usize) {
    if left < right {
        let middle = left + (right - left) / 2;

        sort(arr, left, middle);
        sort(arr, middle + 1, right);

        merge(arr, left, middle, right)
    }
}

fn merge<T: Copy + PartialOrd>(arr: &mut Vec<T>, left: usize, middle: usize, right: usize) {
    let size_1 = middle - left + 1;
    let size_2 = right - middle;

    let mut l_arr: Vec<T> = vec![];
    let mut r_arr: Vec<T> = vec![];

    for i in 0..size_1 {
        l_arr.insert(i, arr[left + i]);
    }

    for j in 0..size_2 {
        r_arr.insert(j, arr[middle + 1 + j]);
    }

    let mut i: usize = 0;
    let mut j: usize = 0;

    let mut k = left;
    while i < size_1 && j < size_2 {
        if l_arr[i] <= r_arr[j] {
            arr[k] = l_arr[i];
            i += 1;
        } else {
            arr[k] = r_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < size_1 {
        arr[k] = l_arr[i];
        i += 1;
        k += 1;
    }

    while j < size_2 {
        arr[k] = r_arr[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{merge_sort, merge_sort_inplace};

    #[test]
    fn merge_sort_test() {
        let arr = vec![4, 6, 9, 2, 1, 3, 5];
        assert_eq!(merge_sort(arr), vec![1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn merge_sort_inplace_test() {
        let mut arr = vec![9, 7, 8, 5, 6, 4, 2, 3, 1];
        merge_sort_inplace(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
