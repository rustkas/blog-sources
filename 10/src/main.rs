extern crate graph;

use graph::Graph;
use graph::AdjacencyList;
use graph::AdjacencyMatrix;
use graph::ListOfEdges;

fn main() {
    let mut adjacency_list = AdjacencyList::<&str>::new();
    create_sample_graph(&mut adjacency_list);
    // AdjacencyList {
    //     map: {
    //         0: {1: 5},
    //         1: {4: 1},
    //         2: {8: 2},
    //         3: {0: 3, 7: 4, 9: 4},
    //         4: {3: 5},
    //         5: {}
    //         6: {0: 1},
    //         7: {2: 2, 4: 2},
    //         8: {2: 5, 5: 1},
    //         9: {6: 2, 8: 2},
    //     },
    //     ...
    // }

    let mut adjacency_matrix = AdjacencyMatrix::<&str>::new();
    create_sample_graph(&mut adjacency_matrix);
    // AdjacencyMatrix {
    //     map: {
    //         0: {0: None,    1: Some(5), 2: None,    3: None,    4: None,    5: None,    6: None,    7: None,    8: None,    9: None   },
    //         1: {0: None,    1: None,    2: None,    3: None,    4: Some(1), 5: None,    6: None,    7: None,    8: None,    9: None   },
    //         2: {0: None,    1: None,    2: None,    3: None,    4: None,    5: None,    6: None,    7: None,    8: Some(5), 9: None   },
    //         3: {0: Some(3), 1: None,    2: None,    3: None,    4: None,    5: None,    6: None,    7: Some(4), 8: None,    9: Some(4)},
    //         4: {0: None,    1: None,    2: None,    3: Some(5), 4: None,    5: None,    6: None,    7: None,    8: None,    9: None   },
    //         5: {0: None,    1: None,    2: None,    3: None,    4: None,    5: None,    6: None,    7: None,    8: None,    9: None   },
    //         6: {0: Some(1), 1: None,    2: None,    3: None,    4: None,    5: None,    6: None,    7: None,    8: None,    9: None   },
    //         7: {0: None,    1: None,    2: Some(2), 3: None,    4: Some(2), 5: None,    6: None,    7: None,    8: None,    9: None   },
    //         8: {0: None,    1: None,    2: Some(5), 3: None,    4: None,    5: Some(1), 6: None,    7: None,    8: None,    9: None   },
    //         9: {0: None,    1: None,    2: None,    3: None,    4: None,    5: None,    6: Some(2), 7: None,    8: Some(2), 9: None   },
    //     },
    //     ...
    // }

    let mut list_of_edges = ListOfEdges::<&str>::new();
    create_sample_graph(&mut list_of_edges);
    // ListOfEdges {
    //     map: [
    //         (0, 0, 5),
    //         (1, 4, 1),
    //         (2, 8, 5),
    //         (3, 0, 3),
    //         (3, 7, 4),
    //         (3, 9, 4),
    //         (4, 3, 5),
    //         (6, 0, 1),
    //         (7, 2, 2),
    //         (7, 4, 2),
    //         (8, 2, 5),
    //         (8, 5, 1),
    //         (9, 6, 2),
    //         (9, 8, 2),
    //     ],
    //     ...
    // }
}

fn create_sample_graph<T: Graph<&'static str>>(graph: &mut T) {
    let key0 = graph.add_node("node0");
    let key1 = graph.add_node("node1");
    let key2 = graph.add_node("node2");
    let key3 = graph.add_node("node3");
    let key4 = graph.add_node("node4");
    let key5 = graph.add_node("node5");
    let key6 = graph.add_node("node6");
    let key7 = graph.add_node("node7");
    let key8 = graph.add_node("node8");
    let key9 = graph.add_node("node9");
    graph.add_edge(key0, key1, 5, false);
    graph.add_edge(key1, key4, 1, false);
    graph.add_edge(key2, key8, 5, true);
    graph.add_edge(key3, key0, 3, false);
    graph.add_edge(key3, key7, 4, false);
    graph.add_edge(key3, key9, 4, false);
    graph.add_edge(key4, key3, 5, false);
    graph.add_edge(key6, key0, 1, false);
    graph.add_edge(key7, key2, 2, false);
    graph.add_edge(key7, key4, 2, false);
    graph.add_edge(key8, key5, 1, false);
    graph.add_edge(key9, key6, 2, false);
    graph.add_edge(key9, key8, 2, false);
}
