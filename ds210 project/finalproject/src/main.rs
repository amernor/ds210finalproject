mod readfile;
mod bfs;

fn main() {
    let file_path = "facebook_combined.txt";
    let n: usize = 4039; 
    let edges = readfile::read_file(file_path);
    let graph = bfs::Graph::create_adjacent(n, &edges);

    //avg distance between all nodes 
    let avg_distance = bfs::compute_average_distance(&graph);
    println!("The average distance between all pairs of nodes: {:.2}\n", avg_distance);

    //degree centrality
    let network_centrality = bfs::compute_degree_centrality(&graph);

    //sort centralities (descending)
    let mut sorted_nodes: Vec<_> = network_centrality.iter().collect();
    sorted_nodes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    //top 5 nodes with the greatest centrality 
    println!("Top 5 users with the greatest degree centrality:");
    for (i, (&node, &centrality_value)) in sorted_nodes.iter().take(5).enumerate() {
        println!("{}. User {}:   Degree Centrality = {} friends", i + 1, node, centrality_value);
    }  
} 


//tests that the distance is > 0.0
#[test]
fn test_average_distance() {
    let test_n: usize = 5;
    let test_edges:  Vec<(usize, usize)> = vec![(0,1),(0,2),(1,2),(2,4),(2,3),(4,3),(4,5),(5,6),(4,6),(6,8),(6,7),(8,7),(1,9)];
    let test_graph = bfs::Graph::create_adjacent(test_n,&test_edges);
    let test_avg_distance = bfs::compute_average_distance(&test_graph);
    assert!(test_avg_distance > 0.0,"Average cannot be calculated");
}

//tests if the centrality values were accurate
#[test]
fn test_degree_centrality() {
    let test_edges_2:  Vec<(usize, usize)> = vec![(0,1),(0,2),(1,2),(2,4),(2,3),(4,3),(4,5),(5,6),(4,6),(6,8),(6,7),(8,7),(1,9)];
    let graph1 = bfs::Graph::create_adjacent(5, &test_edges_2);
    let test_centrality = bfs::compute_degree_centrality(&graph1);

    // check if values match as expected
    assert_eq!(test_centrality[&0], 2.0);
    assert_eq!(test_centrality[&1], 3.0);
    assert_eq!(test_centrality[&2], 4.0);
    assert_eq!(test_centrality[&3], 2.0);
    assert_eq!(test_centrality[&4], 4.0);
    assert_eq!(test_centrality[&5], 2.0);
}
