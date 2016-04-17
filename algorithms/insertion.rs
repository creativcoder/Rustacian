 
pub fn insertion<T>(arr: &mut [T])
	where T: Ord + Copy {
		
	for i in 1..arr.len() {
		let cur_val = arr[i];
		let mut pos = i;
		while pos > 0 && arr[pos-1] > cur_val {
			arr[pos] = arr[pos-1];
			pos = pos-1;
		}
		arr[pos] = cur_val;
	}
	
}

#[test]
fn test_insertion() {
    let mut nums = [231,3,32,34,3,5,4,56,34,3,445,34,646,43,56,4,6,4,1,-23];
    let length = nums.len();
    insertion(&mut nums);
    println!("{:?}",nums );
}