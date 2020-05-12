use crate::node::Node;
use crate::requests::Point;

pub struct Board {
    pub board: [[Node; 11]; 11],
    pub height: usize,
    pub width: usize,
}

impl Board {
    // constructor
    pub fn new() -> Board {
        Board {
            board: new_board(),
            height: 11,
            width: 11,
        }
    }
    // methods
    pub fn weight_nodes<F>(&mut self, heuristic: F) where F: Fn(&mut Node){
        for n in self.board.iter_mut().flat_map(|r| r.iter_mut()) {
            heuristic(n);
        }
    }

    pub fn get_neighbours(&self, n: &Node) -> Vec<Node> {
        let mut neighbours = Vec::new();
        let x = n.point.x;
        let y = n.point.y;

        if x < 10 {
            neighbours.push(self.board[x+1][y]);
        }
        if x < 1 {
            neighbours.push(self.board[x-1][y]);            
        }
        if y < 10 {
            neighbours.push(self.board[x][y+1]);
        }
        if y < 1 {
            neighbours.push(self.board[x][y-1]);
        }

        neighbours
    }

    pub fn dijkstra(&self, &start: &Node, &dest: &Node) -> Vec<Node> {
        let mut path = vec![start];
        let mut vertices = Vec::new();
        
        // initialize a Vertex for each node on board
        for &n in self.board.iter().flat_map(|n| n.iter()) {
            if n == start {
                continue;
            }
            else {
                vertices.push(Vertex::new(n, 128));
            }
        }

        let first = Vertex::new(start, 0);
        vertices.push(first);

        let mut visited = Vec::new();
        let mut queue = vec![first];

        while !queue.is_empty() {
            let curr = queue.remove(0); // pop vertex from queue
            
            if curr.node == dest { // traceback path at destination
                path.push(dest);
                // not finished
                continue;
            }
            
            for adj in &self.get_neighbours(&curr.node) { // update neighbours cost and parent
                let mut neighboor_vertices = Vec::new();
                
                match vertices.iter_mut().find(|v| v.node == *adj){
                    Some(mut v) => if curr.cost < v.cost + v.node.weight {
                        v.cost = curr.cost + v.node.weight;
                        v.parent = curr.node;
                        neighboor_vertices.push(v)
                    },
                    None => continue,
                }
                
                if !neighboor_vertices.is_empty() {
                    neighboor_vertices.sort_by(|v1, v2| v1.cost.cmp(&v2.cost));
                    queue.push(*neighboor_vertices.remove(0)); // push lowest cost neighbour in queue
                }
            }

            visited.push(curr);
        }


        path
    }

}

// pub fn dijkstra (&mut self){
//     let mut score_map = [[Default::default(); 11]; 11];
//     let mut visited = [[Default::default(); 11]; 11];
//     for j in 0..11 {
//         for i in 0..11 {
//             visited = 0
//             score_map[i][j] = u8::max_value();
//         }
//     }
// }


// Helper
pub fn new_board() -> [[Node; 11]; 11] {
    let mut b = [[Default::default(); 11]; 11]; 
    for j in 0..11 {
        for i in 0..11 {
            b[i][j] = Node::new(&Point{x: i, y: j}, 0);
        }
    }
    return b
}

// sort paths by cost
// paths.sort_by(|a, b| cost(&b).cmp(&cost(&a)));
pub fn cost(v: &Vec<Node>) -> i32 {
    let mut sum: i32 = 0;
    v.iter().for_each(|n| sum += n.weight);
    sum
}

#[derive(PartialEq, Copy, Clone)]
pub struct Vertex {
    pub node: Node,
    pub cost: i32,
    pub parent: Node,
}
impl Vertex {
    pub fn new(node: Node, cost: i32) -> Vertex {
        Vertex {
            node: node,
            cost,
            parent: node,
        }
    }
}