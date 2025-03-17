impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut found_indices: Vec<i32> = Vec::new();
        //We need to search all variations of two indices. E.g. for an array 'num's like [1, 2, 3], target = 3, we'd have to check
        // [0, 0], [0, 1], [0, 2], [1, 0], [1, 1], [1, 2], [2, 0], [2, 1], and [2,2] and see if the values of these added together
        // equal the target.
        //Really what we need are unordered pairs of possible indices, add them to a list and find which  one has the target.
        //
        let ordered_indices = generate_ordered_pairs(nums.len() - 1);

        for indices in ordered_indices.iter() {
            let (index1, index2) = indices;
            println!("{}", index1);

            if nums[*index1] + nums[*index2] == target && *index1 != *index2 {
                found_indices.push(*index1 as i32);
                found_indices.push(*index2 as i32);
                return found_indices;
            }
        }

        found_indices
    }
}

fn generate_ordered_pairs(n: usize) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();

    for i in 0..=n {
        for j in 0..=n {
            pairs.push((i, j));
        }
    }

    pairs
}
