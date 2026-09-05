fn prefix_sum(a: &[i64]) -> Vec<i64> {
    let new_len = a.len() + 1;
    let mut result = vec![0; new_len];
    for i in 1..new_len {
        result[i] = result[i - 1] + a[i - 1];
    }
    result
}

fn prefix_sum_2d(grid: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut prefix = vec![vec![0i64; cols + 1]; rows + 1];
    for i in 1..=rows {
        for j in 1..=cols {
            prefix[i][j] =
                prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1] + grid[i - 1][j - 1];
        }
    }
    prefix
}
