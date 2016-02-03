use std::collections::HashMap;

pub type Key = usize;
pub type Weight = usize;

pub trait Graph<N> {
    fn new() -> Self;
    fn add_node(&mut self, node: N) -> Key;
    fn add_edge(&mut self, begin: Key, end: Key, weight: Weight, bidirected: bool);
}



// Adjacency List --------------------------------------------------

#[derive(Debug)]
pub struct AdjacencyList<N> {
    map: HashMap<Key, HashMap<Key, Weight>>,
    list: HashMap<Key, N>,
    next_value: Key,
}

impl<N> AdjacencyList<N> {
    fn get_next_key(&mut self) -> Key {
        let key = self.next_value;
        self.next_value += 1;
        key
    }
}

impl<N> Graph<N> for AdjacencyList<N> {
    fn new() -> Self {
        AdjacencyList {
            map: HashMap::new(),
            list: HashMap::new(),
            next_value: 1,
        }
    }

    fn add_node(&mut self, node: N) -> Key {
        let key = self.get_next_key();
        self.list.insert(key, node);
        self.map.insert(key, HashMap::new());
        key
    }

    fn add_edge(&mut self, begin: Key, end: Key, weight: Weight, bidirected: bool) {
        if let Some(map) = self.map.get_mut(&begin) {
            map.insert(end, weight);
        } else {
            return;
        }

        if bidirected {
            if let Some(map) = self.map.get_mut(&end) {
                map.insert(begin, weight);
            }
        }
    }
}



// Adjacency Matrix --------------------------------------------------

#[derive(Debug)]
pub struct AdjacencyMatrix<N> {
    map: HashMap<Key, HashMap<Key, Option<Weight>>>,
    list: HashMap<Key, N>,
    next_value: Key,
}

impl<N> AdjacencyMatrix<N> {
    fn get_next_key(&mut self) -> Key {
        let key = self.next_value;
        self.next_value += 1;
        key
    }
}

impl<N> Graph<N> for AdjacencyMatrix<N> {
    fn new() -> Self {
        AdjacencyMatrix {
            map: HashMap::new(),
            list: HashMap::new(),
            next_value: 1,
        }
    }

    fn add_node(&mut self, node: N) -> Key {
        let key = self.get_next_key();
        self.list.insert(key, node);
        self.map.insert(key, HashMap::new());

        let mut exist_keys: Vec<usize> = vec![];
        for (item_key, item_map) in &mut self.map {
            item_map.insert(key, None);
            if *item_key != key {
                exist_keys.push(*item_key);
            }
        }

        let mut inserted_map = self.map.get_mut(&key).unwrap();
        for item_key in exist_keys {
            inserted_map.insert(item_key, None);
        }

        key
    }

    fn add_edge(&mut self, begin: Key, end: Key, weight: Weight, bidirected: bool) {
        if let Some(map) = self.map.get_mut(&begin) {
            map.insert(end, Some(weight));
        } else {
            return;
        }

        if bidirected {
            if let Some(map) = self.map.get_mut(&end) {
                map.insert(begin, Some(weight));
            }
        }
    }
}



// List Of Edges --------------------------------------------------

#[derive(Debug)]
pub struct ListOfEdges<N> {
    map: Vec<(Key, Key, Weight)>,
    list: Vec<N>,
}
impl<N> Graph<N> for ListOfEdges<N> {
    fn new() -> Self {
        ListOfEdges {
            map: vec![],
            list: vec![],
        }
    }

    fn add_node(&mut self, node: N) -> Key {
        self.list.push(node);
        self.list.len() - 1
    }

    fn add_edge(&mut self, begin: Key, end: Key, weight: Weight, bidirected: bool) {
        self.map.push((begin, end, weight));
        if bidirected {
            self.map.push((end, end, weight));
        }
    }
}
