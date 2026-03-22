use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mod {
    val: u64,
    m: u64,
}

impl Mod {
    pub fn new(val: u64, m: u64) -> Self {
        Self { val: val % m, m }
    }

    pub fn value(self) -> u64 {
        self.val
    }
}

impl Mul for Mod {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.m, rhs.m);
        let res = (self.val as u128 * rhs.val as u128) % self.m as u128;
        Self {
            val: res as u64,
            m: self.m,
        }
    }
}

fn pow(mut base: Mod, mut exp: u64) -> Mod {
    let mut res = Mod::new(1, base.m);
    while exp > 0 {
        if exp % 2 == 1 {
            res = res * base;
        }
        base = base * base;
        exp /= 2;
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);

    let first_line = match lines.next() {
        Some(line) => line,
        None => return,
    };
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (n, m, modulo) = (parts[0], parts[1], parts[2] as u64);

    let mut adj = vec![vec![]; n];

    for _ in 0..m {
        if let Some(line) = lines.next() {
            let edge: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let u = edge[0] - 1;
            let v = edge[1] - 1;
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    let mut visited = vec![false; n];
    let mut components = Vec::new();

    for i in 0..n {
        if !visited[i] {
            let mut count: u64 = 0;
            let mut q = VecDeque::new();
            q.push_back(i);
            visited[i] = true;
            while let Some(u) = q.pop_front() {
                count += 1;
                for &v in &adj[u] {
                    if !visited[v] {
                        visited[v] = true;
                        q.push_back(v);
                    }
                }
            }
            components.push(count);
        }
    }

    let k = components.len() as u64;

    if k == 1 {
        println!("{}", 1 % modulo);
        return;
    }

    let mut ans = pow(Mod::new(n as u64, modulo), k - 2);
    for &c in &components {
        ans = ans * Mod::new(c, modulo);
    }

    println!("{}", ans.value());
}
