use std::collections::HashMap;

fn main() {
    let mut adjacency_list = AdjacencyList::<&str>::new();
    create_sample_graph(&mut adjacency_list);
    println!("{:?}", adjacency_list);
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
    println!("{:?}", adjacency_matrix);
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
    graph.add_edge(key1, key2, 5);
    graph.add_edge(key2, key5, 1);
    graph.add_edge(key4, key1, 3);
    graph.add_edge(key4, key8, 4);
    graph.add_edge(key4, key10, 4);
    graph.add_edge(key5, key4, 5);
    graph.add_edge(key7, key1, 1);
    graph.add_edge(key8, key3, 2);
    graph.add_edge(key8, key5, 2);
    graph.add_edge(key9, key6, 1);
    graph.add_edge(key10, key7, 2);
    graph.add_edge(key10, key9, 2);
}

type NodeKey = usize;
type EdgeWeight = usize;

trait Graph<T> {
    fn add_node(&mut self, node: T) -> NodeKey;
    fn add_edge(&mut self, begin: NodeKey, end: NodeKey, weight: EdgeWeight);
    fn new() -> Self;
}


// Adjacency List
#[derive(Debug)]
struct AdjacencyList<T> {
    map: HashMap<NodeKey, HashMap<NodeKey, EdgeWeight>>,
    list: HashMap<NodeKey, T>,
    last_key: NodeKey
}

impl<T> AdjacencyList<T> {
    fn get_next_key(&mut self) -> NodeKey {
        self.last_key += 1;
        self.last_key
    }
}

impl<T> Graph<T> for AdjacencyList<T> {
    fn add_node(&mut self, node: T) -> NodeKey {
        let key = self.get_next_key();
        self.list.insert(key, node);
        self.map.insert(key, HashMap::new());
        key
    }

    fn add_edge(&mut self, begin: NodeKey, end: NodeKey, weight: EdgeWeight) {
        self.map.get_mut(&begin).unwrap().insert(end,weight);
    }

    fn new() -> Self {
        AdjacencyList {
            map: HashMap::new(),
            list: HashMap::new(),
            last_key: 0,
        }
    }
}


// Adjacency Matrix
#[derive(Debug)]
struct AdjacencyMatrix<T> {
    map: HashMap<NodeKey, HashMap<NodeKey, Option<EdgeWeight>>>,
    list: HashMap<NodeKey, T>,
    last_key: NodeKey
}

impl<T> AdjacencyMatrix<T> {
    fn get_next_key(&mut self) -> NodeKey {
        self.last_key += 1;
        self.last_key
    }
}

impl<T> Graph<T> for AdjacencyMatrix<T> {
    fn add_node(&mut self, node: T) -> NodeKey {
        let inserted_key = self.get_next_key();
        self.list.insert(inserted_key, node);
        self.map.insert(inserted_key, HashMap::new());

        let mut exist_keys: Vec<usize> = vec![];
        for (key, map) in &mut self.map {
            map.insert(inserted_key, None);
            if *key != inserted_key {
                exist_keys.push(*key);
            }
        }

        let mut inserted_map = self.map.get_mut(&inserted_key).unwrap();
        for key in exist_keys {
            inserted_map.insert(key, None);
        }

        inserted_key
    }

    fn add_edge(&mut self, begin: NodeKey, end: NodeKey, weight: EdgeWeight) {
        self.map.get_mut(&begin).unwrap().insert(end, Some(weight));
    }

    fn new() -> Self {
        AdjacencyMatrix {
            map: HashMap::new(),
            list: HashMap::new(),
            last_key: 0,
        }
    }
}


// List of edges
#[derive(Debug)]
struct AdjacencyMatrix<T> {
    map: Vec<NodeKey, NodeKey, EdgeWeight>,
    list: HashMap<NodeKey, T>,
    last_key: NodeKey
}

impl<T> AdjacencyMatrix<T> {
    fn get_next_key(&mut self) -> NodeKey {
        self.last_key += 1;
        self.last_key
    }
}
