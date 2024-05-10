use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Group {
    id: u32,
}

struct Map {
    n: u32,
    group_dict: HashMap<Position, Group>,
    reward_dict: HashMap<Position, u32>,
    visited_groups: HashSet<Group>,
}

impl Map {
    fn new(
        n: u32,
        group_dict: HashMap<Position, Group>,
        reward_dict: HashMap<Position, u32>,
    ) -> Map {
        Map {
            n,
            group_dict,
            reward_dict,
            visited_groups: HashSet::new(),
        }
    }

    fn visited(&self, position: &Position) -> bool {
        let group = self.group_dict.get(position).unwrap();
        self.visited_groups.contains(group)
    }

    fn visit(&mut self, position: &Position) {
        let group = self.group_dict.get(position).unwrap();
        assert!(!self.visited_groups.contains(group));

        self.visited_groups.insert(*group);
    }

    fn unvisit(&mut self, position: &Position) {
        let group = self.group_dict.get(position).unwrap();
        assert!(self.visited_groups.contains(group));

        self.visited_groups.remove(group);
    }

    fn next_candidates(&self, position: &Position) -> Vec<Position> {
        let mut candidates = Vec::new();
        let x = position.x;
        let y = position.y;
        let n = self.n as usize;
        assert!(n > 0);
        if x > 0 {
            let next = Position { x: x - 1, y };
            if !self.visited(&next) {
                candidates.push(next);
            }
        }
        if y > 0 {
            let next = Position { x, y: y - 1 };
            if !self.visited(&next) {
                candidates.push(next);
            }
        }
        if x < n - 1 {
            let next = Position { x: x + 1, y };
            if !self.visited(&next) {
                candidates.push(next);
            }
        }
        if y < n - 1 {
            let next = Position { x, y: y + 1 };
            if !self.visited(&next) {
                candidates.push(next);
            }
        }
        candidates
    }
}

fn parse_numbers<T: FromStr>(line: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    line.split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<T>().unwrap())
        .collect::<Vec<_>>()
}

struct Rollback {
    position: Position,
}

#[derive(Clone, Debug)]
struct State {
    score: u32,
    history: Vec<Position>,
}

enum StateOrRollback {
    State(State),
    Rollback(Rollback),
}

fn dfs(map: &mut Map, starting_position: &Position) -> Option<State> {
    let mut stack = vec![StateOrRollback::State(State {
        score: 0,
        history: vec![starting_position.clone()],
    })];

    let mut counter = 0_u32;
    let max_counter = 1000 * 10;
    let mut best: Option<State> = None;

    while let Some(s_or_r) = stack.pop() {
        match s_or_r {
            StateOrRollback::State(state) => {
                if let Some(best_state) = &best {
                    if best_state.score < state.score {
                        best = Some(state.clone());
                    }
                } else {
                    best = Some(state.clone());
                }
                counter += 1;
                let position = state.history.last().unwrap().clone();
                let score = state.score;
                map.visit(&position);
                stack.push(StateOrRollback::Rollback(Rollback {
                    position: position.clone(),
                }));

                for next in map.next_candidates(&position) {
                    let reward = map.reward_dict.get(&next).unwrap();
                    let next_score = score + reward;
                    let mut history = state.history.clone();
                    history.push(next);
                    stack.push(StateOrRollback::State(State {
                        score: next_score,
                        history,
                    }));
                }
            }
            StateOrRollback::Rollback(rollback) => {
                map.unvisit(&rollback.position);
            }
        }
        if counter >= max_counter {
            break;
        }
    }

    best
}

fn print_history(state: &State) {
    let history = &state.history;
    for i in 1..history.len() {
        let prev = &history[i - 1];
        let current = &history[i];

        let dx = current.x as i32 - prev.x as i32;
        let dy = current.y as i32 - prev.y as i32;

        if dx < 0 {
            print!("U");
        } else if dx > 0 {
            print!("D");
        } else if dy < 0 {
            print!("L");
        } else if dy > 0 {
            print!("R");
        } else {
            panic!("Invalid move");
        }
    }
    println!();
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let v = parse_numbers(&line);
    assert_eq!(v.len(), 2);
    let starting_position = Position { x: v[0], y: v[1] };

    let mut group_dict = HashMap::new();
    let n = 50_usize;
    for i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let groups = parse_numbers(&line);
        assert_eq!(groups.len(), n);
        for j in 0..n {
            let group = Group { id: groups[j] };
            let position = Position { x: i, y: j };
            group_dict.insert(position, group);
        }
    }

    let mut reward_dict = HashMap::new();
    for i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let rewards = parse_numbers(&line);
        assert_eq!(rewards.len(), n as usize);
        for j in 0..n {
            let position = Position { x: i, y: j };
            reward_dict.insert(position, rewards[j]);
        }
    }

    let mut map = Map::new(50, group_dict, reward_dict);

    let best = dfs(&mut map, &starting_position).unwrap();
    print_history(&best);
}
