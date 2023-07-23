use proconio::input;
use std::convert::TryInto;
use std::collections::VecDeque;
// use std::time::Instant;

fn main() {
    // 時間計測
    // let start_time = Instant::now();

    // データ入力
    input! {
        n: usize,
        m: usize,
        k:usize,
        _xy: [[i32; 2]; n],
        uvw: [[i32; 3]; m],
        _ab: [[i32; 2]; k]
    }

    // 送電マップ
    let mut souden_map = vec![vec![0; n]; n];
    for i in 0..m {
        let u = uvw[i as usize][0] - 1;
        let v = uvw[i as usize][1] - 1;
        souden_map[u as usize][v as usize] = 1;
        souden_map[v as usize][u as usize] = 1;
    }

    let mut n_ans = vec![0; m];
    let mut n_list = VecDeque::new();
    n_list.push_back(0);
    let mut flag_map = vec![0; n];
    while n_list.len() != 0 {
        let first_element = n_list[0];
        flag_map[first_element as usize] = 1;
        n_list.pop_front();
        
        for i in 0..n {
            if flag_map[i as usize] == 0 && souden_map[i as usize][first_element as usize] == 1 {
                n_list.push_back(i);
                let fe_u = first_element + 1;
                let i_u = i + 1;

                for j in 0..m {
                    if uvw[j][0] == fe_u.try_into().unwrap()  && uvw[j][1] == i_u.try_into().unwrap(){
                        n_ans[j] = 1;
                    }
                }
                for j in 0..m {
                    if uvw[j][1] == fe_u.try_into().unwrap()  && uvw[j][0] == i_u.try_into().unwrap(){
                        n_ans[j] = 1;
                    }
                }
            }
        }
    }
    for i in &n_ans {
        print!("{} ", i)
    }
    // let mut end_time = Instant::now();
    // let mut elapsed_time = end_time.duration_since(start_time);
    // let mut elapsed_secs = elapsed_time.as_secs();

    
    // while elapsed_secs < 4000 {
        
    //     end_time = Instant::now();
    //     elapsed_time = end_time.duration_since(start_time);
    //     elapsed_secs = elapsed_time.as_secs();
    // }
    
}
