impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut a:Vec<i32> = vec![];
        let mut asteroids = asteroids;
        let mut l = asteroids.len();
        let mut c = 0;
        let mut flag = false;
        loop {
            c+=1;
            println!("*loop:{:?}*",c);
            a.clear();
            for i in 0..(l-1){
                if flag{
                    flag = false;
                    continue;
                }
                if (asteroids[i] > 0) && (asteroids[i+1] < 0){
                    if asteroids[i] > (-asteroids[i+1]){
                        a.push(asteroids[i]);
                        flag = true;
                    }else if asteroids[i] < (-asteroids[i+1]){
                        a.push(asteroids[i+1]);
                        flag = true;
                    }else if (asteroids[i] + asteroids[i+1]) == 0{
                        flag = true;;
                    }     
                }
                else{
                    a.push(asteroids[i]);
                }
            }
            flag = false;
            if l >= 2 && ((asteroids[l-1] > 0) || (asteroids[l-2] < 0)) && !a.is_empty(){
                a.push(asteroids[l-1]);
            }
            if a.is_empty() && l > 2 && l < 4{
                a.push(asteroids[l-1]);
            }
            
            if (l == a.len()) || (a.is_empty()) || l == 2 {
                break
            }
            l = a.len();
            asteroids = a.clone();
            if l == 1{
                break
            }     
        }     
        a
    }
}
