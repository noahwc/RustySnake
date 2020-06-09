use crate::{graph, node, requests, responses};

pub fn get_move(turn: requests::Turn) -> Option<responses::Move> {
    let mut graph = graph::Graph::new(&turn);
    let mut paths = Vec::new();
    // refactor into board constructor?
    graph.board[turn.you.body.first().unwrap().index(graph.width)].has_head = true;
    graph.board[turn.you.body.last().unwrap().index(graph.width)].has_tail = true;

    for snake in turn.board.snakes {
        for point in snake.body {
            graph.board[point.index(graph.width)].has_snake = true
        }
    }
    for point in turn.board.food {
        graph.board[point.index(graph.width)].has_food = true
    }

    let empty_weight = 1;
    let snake_weight = 122;
    let head_weight = 0;

    let weighting_heuristic = |n: node::Node| -> (i32, bool) {
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

    graph.weight_nodes(weighting_heuristic);

    // run dijstra here
    paths.sort_by(|a, b| cost(a).cmp(&cost(b)));

    for path in &paths {
        if graph.is_safe(path, turn.you.body.len()) {
            return Some(responses::Move::new(get_direction(path)));
        }
    }
    return None;
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

fn cost(v: &Vec<node::Node>) -> i32 {
    let mut sum: i32 = 0;
    v.iter().for_each(|n| sum += n.weight);
    sum
}
