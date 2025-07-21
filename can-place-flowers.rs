impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {

        let mut fc = flowerbed.clone();
        let num = flowerbed.len();
        let mut n = n.clone();

        if n == 0 {return true}

        if num == 1{return fc[0] == 0 && n<= 1}

        for i in 0..num{
            if fc[i] == 0{
                let left_ok = i == 0 || fc[i-1] == 0;
                let right_ok = i == num -1 || fc[i+1] == 0 ;

                if left_ok && right_ok{
                    fc[i] = 1;
                    n -= 1;
                    if n == 0 {return true
                    }
                }
            }
        }
         n <= 0
    }
}
