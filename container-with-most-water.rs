impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        
        while left < right {
            let h = std::cmp::min(height[left], height[right]);
            let width = (right - left) as i32;
            let area = h * width;
            
            if area > max_area {
                max_area = area;
            }
            
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        
        max_area
    }
}
