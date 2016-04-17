use std::cmp::Ord;
use std::fmt::Debug;

mod insertion;

use insertion::insertion as insertion_sort;

fn qsort<T>(arr: &mut [T],len:usize)
	where T: Ord + Copy + Debug {
	quicksort(arr,0,len);
}

fn quicksort<T>(arr:&mut [T],low:usize,high:usize)
	where T: Ord + Copy + Debug {
	if low < high {
		let split = partition(arr,low,high);
		if high - low < 100 {
			insertion_sort(arr);
		}
		quicksort(arr,low,split);
		quicksort(arr,split+1,high);
	}
}

fn partition<T>(arr: &mut [T], low:usize,high:usize) -> usize
	where T: Ord + Copy + Debug {
	
	let mid = ((high + low) / 2) as usize;
	let mut pivot = high;
	if arr[low] < arr[mid] {
		pivot = mid;
	}
	else if arr[low] < arr[high] {
		pivot = low;
	}
	
	let pivot_val = arr[pivot];
	let temp = arr[pivot];
	arr[pivot] = arr[low];
	arr[low] = temp;

	let mut border = low;
	
	for i in low..high+1 {
		if arr[i] < pivot_val {
			border += 1;
			let temp = arr[i];
			arr[i] = arr[border];
			arr[border] = temp;
			
		}
	}
	let temp = arr[low];
	arr[low] = arr[border];
	arr[border] = temp;
	
	return border;
}


fn main() {
    let mut nums = [231,3,32,34,3,5,4,56,34,3,445,34,646,43,56,4,6,4,1,-23];
    let length = nums.len();
    qsort(&mut nums,length-1);
    println!("{:?}",nums );
}