fn main() {
    let vec = vec![1, 2, 1, 4, 1, 2, 10, 2];
    let sorted_vec = bubble_sort(vec);
    println!("{:?}", sorted_vec);
}

// バブルソート
fn bubble_sort(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();
    let vec_len = vec.len();
    for i in 0..vec_len - 1 {
        // 一番後ろからソートする（最小値が左端に来るようにする）
        for j in (i..vec_len - 1).rev() {
            // 隣同士の大小を比較し、小さい方を左側に入れ替える（同値の場合は入れ替えない）
            if vec[j] > vec[j + 1] {
                let temp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = temp;
            }
        }
    }
    // vecをreturn
    vec
}
