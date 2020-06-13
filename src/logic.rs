use crate::{graph, node, requests, responses};

pub fn get_move(turn: requests::Turn) -> Option<responses::Move> {
    let mut graph = graph::Graph::new(&turn);

    graph.board[graph::index(graph.width, turn.you.body.first().unwrap())].has_head = true;
    graph.board[graph::index(graph.width, turn.you.body.last().unwrap())].has_tail = true;

    for snake in turn.board.snakes {
        for point in &snake.body {
            graph.board[graph::index(graph.width, point)].has_snake = true
        }
    }
    
    for point in &turn.board.food {
        graph.board[graph::index(graph.width, point)].has_food = true
    }

    let empty_weight = 1;
    let snake_weight = 122;
    let head_weight = 0;

    let weighting_heuristic = |n: node::Node| -> (usize, bool) {
        if n.has_snake {
            if n.has_head {
                return (head_weight, false);
            } else {
                return (snake_weight, false);
            }
        }
        if n.has_food {
            return (empty_weight, true);
        }
        (empty_weight, false)
    };

    let targets = graph.weight_nodes(weighting_heuristic);
    let mut paths = Vec::new();

    let head = graph.board[ graph::index(graph.width, turn.you.body.first().unwrap()) ];

    for target in targets {
        match graph.bfs(head, target) {
            Some(path) => paths.push(path),
            None => ()
        }
    }
    
    paths.sort_by(|a, b| weight(a).cmp(&weight(b)));

    for path in &paths {
        if graph.is_safe(path, turn.you.body.len()) {
            return Some(responses::Move::new(get_direction(path)));
        }
    }
    
    match graph.wait(&head, &turn.you.body) {
        Some(path) => Some(responses::Move::new(get_direction(&path))),
        None => None
    }
}

fn get_direction(path: &Vec<node::Node>) -> responses::Direction {
    let a = path.first().expect("no first node!");
    let b = path.iter().nth(1).expect("no second node!");
    if b.point.x > a.point.x {
        return responses::Direction::Right;
    }
    if b.point.x < a.point.x {
        return responses::Direction::Left;
    }
    if b.point.y > a.point.y {
        return responses::Direction::Down;
    } else {
        return responses::Direction::Up;
    }
}

fn weight(v: &Vec<node::Node>) -> usize {
    let mut sum: usize = 0;
    v.iter().for_each(|n| sum += n.weight);
    sum
}
