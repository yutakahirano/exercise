use std::io;


fn max_flow(table: &Vec<Vec<i32>>, src: usize, dest: usize) -> u32 {
    let n = table.len();
    let mut flow = vec![vec![0; n]; n];
    let mut result = 0;
    loop {
        let mut visited = vec![false; n];
        let mut stack = vec![src];
        visited[src] = true;
        let mut min = std::i32::MAX;
        let mut previous = vec![None; n];
        while let Some(u) = stack.pop() {
            if u == dest {
                break;
            }
            for v in 0..n {
                if !visited[v] && table[u][v] > flow[u][v] {
                    stack.push(v);
                    visited[v] = true;
                    previous[v] = Some(u);
                    min = std::cmp::min(min, table[u][v] - flow[u][v]);
                }
            }
        }
        if previous[dest].is_none() {
            break;
        }
        result += min;
        let mut v = dest;
        while v != src {
            let u = previous[v].unwrap();
            flow[u][v] += min;
            flow[v][u] -= min;
            v = u;
        }
    }
    result as u32
}

// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bp
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let ss = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    assert!(ss.len() == 2);
    let n = ss[0] as usize;
    let m = ss[1];

    let mut table: Vec<Vec<i32>> = vec![vec![0; n]; n];
    for _ in 0..m {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let ss = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        assert!(ss.len() == 3);

        let a = ss[0] as usize;
        let b = ss[1] as usize;
        let c = ss[2];
        assert!(a > 0);
        assert!(b > 0);
        table[a - 1][b - 1] = c;
    }
    println!("{}", max_flow(&table, 0, n - 1));
}