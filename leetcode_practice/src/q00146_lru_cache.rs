use std::collections::hash_map::Entry;
use std::collections::HashMap;

type LRUKey = i32;
type LRUValue = i32;
type LRUNodeIndex = usize;

struct LRUNode {
    key: LRUKey,
    value: LRUValue,
    less_recent_i: Option<LRUNodeIndex>,
    more_recent_i: Option<LRUNodeIndex>,
}
struct LRUCache {
    map: HashMap<LRUKey, LRUNodeIndex>,
    store: Vec<LRUNode>,
    capacity: usize,
    current_size: usize,
    least_recent_i: Option<LRUNodeIndex>,
    most_recent_i: Option<LRUNodeIndex>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 *
 * Your LRUCache object will be instantiated and called as such:
 * ```
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 * ```
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            map: HashMap::new(),
            store: Vec::with_capacity(capacity as usize),
            capacity: capacity as usize,
            current_size: 0,
            least_recent_i: Some(0),
            most_recent_i: None,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&store_index) = self.map.get(&key) {
            self.make_most_recently_used(store_index);
            self.store[store_index].value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: LRUKey, value: LRUValue) {
        match self.map.entry(key) {
            Entry::Occupied(entry) => {
                let store_index = *entry.get();
                self.make_most_recently_used(store_index);
                self.store[store_index].value = value;
            }
            Entry::Vacant(entry) => {
                // if we haven't reached capacity yet
                if self.current_size < self.capacity {
                    self.store.push(LRUNode {
                        key,
                        value,
                        less_recent_i: self.most_recent_i,
                        more_recent_i: None,
                    });
                    let store_index = self.store.len() - 1;
                    if let Some(i) = self.most_recent_i {
                        self.store[i].more_recent_i = Some(store_index);
                    }
                    self.most_recent_i = Some(store_index);
                    entry.insert(store_index);
                    self.current_size += 1;
                } else {
                    // replace the least recently used (first) item
                    let store_index = self.least_recent_i.unwrap(); // if we're at capacity and least_recent_i is None, something has gone seriously wrong
                    entry.insert(store_index);
                    self.map.remove(&self.store[store_index].key);
                    self.store[store_index].key = key;
                    self.store[store_index].value = value;
                    self.make_most_recently_used(store_index)
                }
            }
        }
    }

    fn make_most_recently_used(&mut self, store_index: LRUNodeIndex) {
        // if key is already most recently used, return
        if Some(store_index) == self.most_recent_i {
            return;
        }
        // if this was the least recently used, make least_recent_i point to the next node
        if Some(store_index) == self.least_recent_i {
            self.least_recent_i = self.store[store_index].more_recent_i;
        }
        // unlink from prev and next
        let mru = self.store[store_index].more_recent_i; // we know we're not the most recently used, so there must be a more recently used
        let lru = self.store[store_index].less_recent_i;
        if let Some(i) = lru {
            self.store[i].more_recent_i = mru;
        }
        if let Some(i) = mru {
            self.store[i].less_recent_i = lru;
        }
        // make most recently used node point to this as more recently used (and vice versa)
        if let Some(i) = self.most_recent_i {
            self.store[i].more_recent_i = Some(store_index);
        }
        self.store[store_index].less_recent_i = self.most_recent_i;
        // update most_recently_used to point to this index
        self.most_recent_i = Some(store_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_lru_cache() {
        // let ops = vec!["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"];
        // let args = vec![[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1); // cache is {1=1}
        lru_cache.put(2, 2); // cache is {1=1, 2=2}
        assert_eq!(lru_cache.get(1), 1); // return 1
        lru_cache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        assert_eq!(lru_cache.get(2), -1); // returns -1 (not found)
        lru_cache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        assert_eq!(lru_cache.get(1), -1); // return -1 (not found)
        assert_eq!(lru_cache.get(3), 3); // return 3
        assert_eq!(lru_cache.get(4), 4); // return 4
    }

    enum Command {
        LRUCache(i32),
        Put(i32, i32),
        Get(i32),
    }
    enum Args {
        One([i32; 1]),
        Two([i32; 2]),
    }

    #[rstest]
    #[case(
        vec!["LRUCache","put","put","get","put","get","put","get","get","get"],
        "[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]",
        "[null,null,null,1,null,-1,null,-1,3,4]"
    )]
    #[case(
        vec!["LRUCache","put","put","put","put","put","get","put","get","get","put","get","put","put","put","get","put","get","get","get","get","put","put","get","get","get","put","put","get","put","get","put","get","get","get","put","put","put","get","put","get","get","put","put","get","put","put","put","put","get","put","put","get","put","put","get","put","put","put","put","put","get","put","put","get","put","get","get","get","put","get","get","put","put","put","put","get","put","put","put","put","get","get","get","put","put","put","get","put","put","put","get","put","put","put","get","get","get","put","put","put","put","get","put","put","put","put","put","put","put"],
        "[10],[10,13],[3,17],[6,11],[10,5],[9,10],[13],[2,19],[2],[3],[5,25],[8],[9,22],[5,5],[1,30],[11],[9,12],[7],[5],[8],[9],[4,30],[9,3],[9],[10],[10],[6,14],[3,1],[3],[10,11],[8],[2,14],[1],[5],[4],[11,4],[12,24],[5,18],[13],[7,23],[8],[12],[3,27],[2,12],[5],[2,9],[13,4],[8,18],[1,7],[6],[9,29],[8,21],[5],[6,30],[1,12],[10],[4,15],[7,22],[11,26],[8,17],[9,29],[5],[3,4],[11,30],[12],[4,29],[3],[9],[6],[3,4],[1],[10],[3,29],[10,28],[1,20],[11,13],[3],[3,12],[3,8],[10,9],[3,26],[8],[7],[5],[13,17],[2,27],[11,15],[12],[9,19],[2,15],[3,16],[1],[12,17],[9,1],[6,19],[4],[5],[5],[8,1],[11,7],[5,2],[9,28],[1],[2,2],[7,4],[4,22],[7,24],[9,26],[13,28],[11,26]",
        "[null,null,null,null,null,null,-1,null,19,17,null,-1,null,null,null,-1,null,-1,5,-1,12,null,null,3,5,5,null,null,1,null,-1,null,30,5,30,null,null,null,-1,null,-1,24,null,null,18,null,null,null,null,-1,null,null,18,null,null,-1,null,null,null,null,null,18,null,null,-1,null,4,29,30,null,12,-1,null,null,null,null,29,null,null,null,null,17,22,18,null,null,null,-1,null,null,null,20,null,null,null,-1,18,18,null,null,null,null,20,null,null,null,null,null,null,null]"
    )]
    fn test(#[case] commands: Vec<&str>, #[case] args: &str, #[case] expected: &str) {
        let mut lru_cache = LRUCache::new(0);
        let result = commands
            .iter()
            .zip(args[1..args.len() - 1].split("],["))
            .map(|(&command, args)| -> Command {
                match command {
                    "LRUCache" => Command::LRUCache(args.parse::<i32>().unwrap()),
                    "put" => {
                        let args = args
                            .split(',')
                            .map(|arg| arg.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                        Command::Put(args[0], args[1])
                    }
                    "get" => Command::Get(args.parse::<i32>().unwrap()),
                    _ => panic!("Invalid command"),
                }
            })
            .map(|command| match command {
                Command::LRUCache(capacity) => {
                    lru_cache = LRUCache::new(capacity);
                    "null".to_string()
                }
                Command::Put(key, value) => {
                    lru_cache.put(key, value);
                    "null".to_string()
                }
                Command::Get(key) => {
                    let value = lru_cache.get(key);
                    value.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        assert_eq!(result, expected[1..expected.len() - 1].replace(",", "\n"));
    }
}
