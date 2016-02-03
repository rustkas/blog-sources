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
    //         1:  {2: 5},
    //         2:  {5: 1},
    //         3:  {},
    //         4:  {1: 3, 8: 4, 10: 4},
    //         5:  {4: 5},
    //         6:  {}
    //         7:  {1: 1},
    //         8:  {3: 2, 5: 2},
    //         9:  {6: 1},
    //         10: {7: 2, 9: 2},
    //     },
    //     ...
    // }

    let mut adjacency_matrix = AdjacencyMatrix::<&str>::new();
    create_sample_graph(&mut adjacency_matrix);
    // AdjacencyMatrix {
    //     map: {
    //         1:  {1: None,    2: Some(5), 3: None,    4: None,    5: None,    6: None,    7: None,    8: None,    9: None,    10: None   },
    //         2:  {1: None,    2: None,    3: None,    4: None,    5: Some(1), 6: None,    7: None,    8: None,    9: None,    10: None   },
    //         3:  {1: None,    2: None,    3: None,    4: None,    5: None,    6: None,    7: None,    8: None,    9: None,    10: None   },
    //         4:  {1: Some(3), 2: None,    3: None,    4: None,    5: None,    6: None,    7: None,    8: Some(4), 9: None,    10: Some(4)},
    //         5:  {1: None,    2: None,    3: None,    4: Some(5), 5: None,    6: None,    7: None,    8: None,    9: None,    10: None   },
    //         6:  {1: None,    2: None,    3: None,    4: None,    5: None,    6: None,    7: None,    8: None,    9: None,    10: None   },
    //         7:  {1: Some(1), 2: None,    3: None,    4: None,    5: None,    6: None,    7: None,    8: None,    9: None,    10: None   },
    //         8:  {1: None,    2: None,    3: Some(2), 4: None,    5: Some(2), 6: None,    7: None,    8: None,    9: None,    10: None   },
    //         9:  {1: None,    2: None,    3: None,    4: None,    5: None,    6: Some(1), 7: None,    8: None,    9: None,    10: None   },
    //         10: {1: None,    2: None,    3: None,    4: None,    5: None,    6: None,    7: Some(2), 8: None,    9: Some(2), 10: None   },
    //     },
    //     ...
    // }

    let mut list_of_edges = ListOfEdges::<&str>::new();
    create_sample_graph(&mut list_of_edges);
    // ListOfEdges {
    //     map: [
    //         (0, 0, 5),
    //         (1, 4, 1),
    //         (3, 0, 3),
    //         (3, 7, 4),
    //         (3, 9, 4),
    //         (4, 3, 5),
    //         (6, 0, 1),
    //         (7, 2, 2),
    //         (7, 4, 2),
    //         (8, 5, 1),
    //         (9, 6, 2),
    //         (9, 8, 2),
    //     ],
    //     ...
    // }
}

fn create_sample_graph<T: Graph<&'static str>>(graph: &mut T) {
    let key1 = graph.add_node("node1");
    let key2 = graph.add_node("node2");
    let key3 = graph.add_node("node3");
    let key4 = graph.add_node("node4");
    let key5 = graph.add_node("node5");
    let key6 = graph.add_node("node6");
    let key7 = graph.add_node("node7");
    let key8 = graph.add_node("node8");
    let key9 = graph.add_node("node9");
    let key10 = graph.add_node("node10");
    graph.add_edge(key1, key2, 5, false);
    graph.add_edge(key2, key5, 1, false);
    graph.add_edge(key4, key1, 3, false);
    graph.add_edge(key4, key8, 4, false);
    graph.add_edge(key4, key10, 4, false);
    graph.add_edge(key5, key4, 5, false);
    graph.add_edge(key7, key1, 1, false);
    graph.add_edge(key8, key3, 2, false);
    graph.add_edge(key8, key5, 2, false);
    graph.add_edge(key9, key6, 1, false);
    graph.add_edge(key10, key7, 2, false);
    graph.add_edge(key10, key9, 2, false);
}
