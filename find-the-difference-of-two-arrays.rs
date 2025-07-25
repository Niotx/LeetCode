impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        vec![
            difference(nums1.clone(), nums2.clone()),
            difference(nums2, nums1),
        ]
    }
}
fn difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>{
    let mut v = vec![];
    for i in nums1 {
        if !nums2.contains(&i) && !v.contains(&i) {
            v.push(i);
        }
    }
    v
}
