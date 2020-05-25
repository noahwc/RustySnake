use crate::{responses, requests, game, node};

pub fn get_move (turn: requests::Turn) -> responses::Move {
    // PREREQS //
    let mut game = game::Game::new(&turn); // new game instance
    let mut paths = Vec::new();
    let head = *turn.you.body.first().expect("no head!");
    
    // EARLY GAME //
    let empty_weight = 1;
    let snake_weight = 127;
    let head_weight = 0;
    
    let weighting_heuristic = |n: &mut node::Node| -> i32 {
        if n.has_snake(&turn) {
            if n.point == head {
                return head_weight
            } else {
                return snake_weight
            }
        } if n.has_food(&turn) {
            n.target = true;
        }
        return empty_weight
    };
    // MID GAME //

    // LATE GAME //
    
    // PATHS //
    game.graph.weight_nodes(weighting_heuristic);
    game.graph.djikstra(head);
    for point in &game.graph.targets {
        let node = game.graph.board[point.x][point.y];
        match game.graph.path_to(node) {
            Some(path) => paths.push(path),
            None => (),
        }
    }
    paths.sort_by(|a,b| weight(a).cmp(&weight(b)));
    // ADD FLOOD FILL CHECK HERE //
    if paths.is_empty() {
        return responses::Move::new(responses::Direction::Right)   // return default direction
    } else {
        // println!("{:#?}", paths.first().expect("no path in paths!").last().expect("no node in path")); 
        return responses::Move::new(get_direction(paths.first().expect("no path in paths!")))
    }

}

fn get_direction(path: &Vec<node::Node>) -> responses::Direction {
    let a = path.first().expect("no first node!");
    let b = path.iter().nth(1).expect("no second node!");
    if b.point.x > a.point.x {
        return responses::Direction::Right
    } if b.point.x < a.point.x {
        return responses::Direction::Left
    } if b.point.y > a.point.y {
        return responses::Direction::Down
    } else {
        return responses::Direction::Up
    }
}

pub fn weight(v: &Vec<node::Node>) -> i32 {
    let mut sum: i32 = 0;
    v.iter().for_each(|n| sum += n.weight);
    sum
}