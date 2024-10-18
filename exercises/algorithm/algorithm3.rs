/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T>(array: &mut [T])
where
    T: PartialOrd + Copy,
{
    quick_sort(array, 0, array.len() as isize - 1);
}

fn quick_sort<T>(array: &mut [T], low: isize, high: isize)
where
    T: PartialOrd + Copy,
{
    if low < high {
        let pivot_index = partition(array, low, high);
        if pivot_index > low {
            quick_sort(array, low, pivot_index - 1);
        }
        quick_sort(array, pivot_index + 1, high);
    }
}

fn partition<T>(array: &mut [T], low: isize, high: isize) -> isize
where
    T: PartialOrd + Copy,
{
    let pivot = array[high as usize];
    let mut i = low - 1;
    for j in low..high {
        if array[j as usize] <= pivot {
            i += 1;
            if i != j {
                array.swap(i as usize, j as usize);
            }
        }
    }
    // 将 i + 1 转换为 usize，如果转换失败，则 panic
    let final_index = (i + 1).try_into().expect("Index out of bounds");
    array.swap(final_index, high as usize);
    i + 1
}