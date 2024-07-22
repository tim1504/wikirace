use std::collections::{VecDeque, HashMap};
use crate::graph::Graph;

/// Given a start and end page, find the shortest path connecting them.
pub fn find_shortest_path(start: String, end: String, graph: &Graph) -> Result<Vec<String>, String> {
    
    // Check whether the start and end pages exist.
    let start_id = match graph.get_id(&start) {
        Some(id) => id,
        None => return Err(format!("Page {} does not exist", start)),
    };
    let end_id = match graph.get_id(&end) {
        Some(id) => id,
        None => return Err(format!("Page {} does not exist", end)),
    };

    if start_id == end_id {
        return Err(format!("Start and end pages are the same"));
    }

    let mut queue = VecDeque::new();
    queue.push_back((start_id, true));
    queue.push_back((end_id, false));

    let mut visited = HashMap::new();
    visited.insert(start_id, (start_id, true));
    visited.insert(end_id, (end_id, false));

    // Bidirectional Breadth First Search.
    while let Some((c, d)) = queue.pop_front() {
        let edges = graph.get_edges(&c, d);
        for e in edges {
            if visited.contains_key(&e) {
                if visited[&e].1 != d{
                    // Path was found.
                    // Reconstruct the path.
                    let mut path = VecDeque::new();
                    let mut a = c;
                    while a != start_id && a != end_id {
                        if d {path.push_front(graph.get_name(&a).unwrap());}
                        else {path.push_back(graph.get_name(&a).unwrap());}
                        a = visited[&a].0;
                    }
                    let mut a = e;
                    while a != end_id && a != start_id {
                        if d {path.push_front(graph.get_name(&a).unwrap());}
                        else {path.push_back(graph.get_name(&a).unwrap());}
                        a = visited[&a].0; 
                    }
                    path.push_front(start.to_string());
                    path.push_back(end.to_string());
                    return Ok(path.into_iter().collect());
                }
                continue;
            }
            queue.push_back((e, d));
            visited.insert(e, (c, d));
        }
    }
    
    Err(format!("A path between {} and {} does not exist", start, end))

}