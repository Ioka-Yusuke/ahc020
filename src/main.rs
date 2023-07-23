use proconio::input;
use std::time::Instant;

fn main() {
    // 時間計測
    let start_time = Instant::now();

    // データ入力
    input! {
        n: usize,
        m: usize,
        k:usize,
        xy: [[i32; 2]; n],
        _uvw: [[i32; 3]; m],
        ab: [[i32; 2]; k]
    }
    // map 
    let mut p_ans = vec![5000; n];

    // 住民が受信している電波塔
    let mut denpa = vec![Vec::new(); k];

    for i in 0..n {
        let x = xy[i][0];
        let y = xy[i][1];

        for j in 0..k {
            let a = ab[j][0];
            let b = ab[j][1];
            let xa = x - a;
            let yb = y - b;
            let lengh = xa.pow(2) + yb.pow(2);
            if lengh < 25000000 {
                denpa[j].push(i);
            }
        }
    }

    // 計測部分
    let mut end_time = Instant::now();
    let mut elapsed_time = end_time.duration_since(start_time);
    let mut elapsed_secs = elapsed_time.as_secs();

    let mut i_cnt: usize = 0;
    let mut all_sum = 0;
    for i in 0..k {
        all_sum += denpa.len();
    }
    println!("{}", all_sum);
    while elapsed_secs < 2 {
        i_cnt += 1;
        i_cnt %= n;
        let x = xy[i_cnt][0];
        let y = xy[i_cnt][1];        

        let mut new_denpa = denpa[..].to_owned();
        for i in 0..k {
            new_denpa[i].retain(|&x| x != i_cnt);
        }

        for j in 0..k {
            let a = ab[j][0];
            let b = ab[j][1];
            let xa = x - a;
            let yb = y - b;
            let lengh = xa.pow(2) + yb.pow(2);
            if lengh < (p_ans[i_cnt]-100)*(p_ans[i_cnt]-100) {
                new_denpa[j].push(i_cnt);
            }
        }

        let mut k_cnt: usize = 0;
        for i in 0..k {
            if new_denpa[i].len() != 0 {
                k_cnt += 1
            }
        }
        
        if k_cnt == k && p_ans[i_cnt] - 100 >= 0 {
            p_ans[i_cnt] -= 100;
            denpa = new_denpa; 
        }

        // 計測
        end_time = Instant::now();
        elapsed_time = end_time.duration_since(start_time);
        elapsed_secs = elapsed_time.as_secs();
    }
    
    for i in &p_ans {
        print!("{} ", i);
    }
    println!("");

    for _ in 0..m {
        print!("{} ", 1);
    }
}
