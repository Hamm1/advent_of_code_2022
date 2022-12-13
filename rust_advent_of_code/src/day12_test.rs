
// fn bfs(grid: Vec<Vec<i32>>) -> bool {
//     let mut queue = std::collections::VecDeque::new();
//     let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

//     // Start from the top-left corner
//     queue.push_back((0, 0));
//     visited[0][0] = true;

//     // Neighbors in a grid
//     let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

//     while !queue.is_empty() {
//         let (x, y) = queue.pop_front().unwrap();

//         // Check if the current number is increasing
//         if x > 0 && y > 0 && grid[x][y] > grid[x - 1][y] && grid[x][y] > grid[x][y - 1] {
//             return true;
//         }

//         // Add unvisited neighbors to the queue
//         for (dx, dy) in dirs.iter() {
//             println!("{},{}",dx,dy);
//             let nx = x.to_string().parse::<i32>().unwrap() + dx;
//             let ny = y.to_string().parse::<i32>().unwrap() + dy;

//             if nx >= 0 && nx < grid.len().to_string().parse::<i32>().unwrap() && ny >= 0 && ny < grid[0].len().to_string().parse::<i32>().unwrap() && !visited[nx][ny] {
//                 queue.push_back((nx, ny));
//                 visited[nx.to_string().parse::<i32>().unwrap()][ny] = true;
//             }
//         }
//     }

//     // Return false if no increasing numbers were found
//     false
// }

// pub fn day12(){
//     let eth = vec![
//         vec![0,1,2,17,16,15,14,13],
//         vec![1,2,3,18,25,24,24,12],
//         vec![1,3,3,19,26,27,24,11],
//         vec![1,3,3,20,21,22,23,10],
//         vec![1,2,4,5,6,7,8,9],
        
//     ];
//     bfs(eth);
// }