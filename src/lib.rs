pub mod hashlrurs {
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct HashLRU {
        size: u128,
        max: u128,
        // NOTE: Currently only supports u128 due to to it only being used
        //          in hyperswarm to hold integeters here:
        // https://github.com/hyperswarm/dht/blob/c8bbe643dac374d9c7cf92d723b732a980c564bd/index.js#L36
        cache1: HashMap<u128, u128>,
        cache2: HashMap<u128, u128>,
        state: bool,
    }

    impl HashLRU {
        pub fn new(&self, max: u128) -> HashLRU {
            HashLRU {
                size : 0,
                max,
                cache1: HashMap::with_capacity(max as usize),
                cache2: HashMap::with_capacity(max as usize),
                state: true,
            }
        }
        pub fn has(&self, key: u128) -> bool {
            self.cache1.contains_key(&key) || self.cache2.contains_key(&key)
        }

        pub fn remove(&mut self, key: u128) {
            self.cache1.remove(&key).unwrap();
            self.cache2.remove(&key).unwrap();
        }

        pub fn get(&mut self, key: u128) -> Option<u128> {
            if self.state {
                match self.cache1.get(&key) {
                    Some(&v) => return Some(v),
                    None => (),
                }
                let result = self.cache2.get(&key);
                match result {
                    Some(&v) => {
                        {self.update(key, v);}
                        return Some(v);
                    }
                    None => None,
                }
            } else {
                match self.cache2.get(&key) {
                    Some(&v) => return Some(v),
                    None => (),
                }
                let result = self.cache1.get(&key);
                match result {
                    Some(&v) => {
                        {self.update(key, v);}
                        return Some(v);
                    }
                    None => None,
                }
            }
        }

        pub fn set(&mut self, key: u128, value: u128) {
            if self.state {
            match self.cache1.get(&key) {
                Some(_v) => {self.cache1.insert(key, value);},
                None => {self.update(key, value);},
            };
        } else {
            match self.cache2.get(&key) {
                Some(_v) => {self.cache2.insert(key, value);},
                None => {self.update(key, value);},
            };
        }
        }

        pub fn update(&mut self, key: u128, value: u128) {
            if self.state {
                self.cache1.insert(key, value);
                self.size += 1;
                if self.size >= self.max {
                    self.size = 0;
                    self.state = !self.state;
                    self.cache2.clear();
                }
            } else {
                self.cache2.insert(key, value);
                self.size += 1;
                if self.size >= self.max {
                    self.size = 0;
                    self.state = !self.state;
                    self.cache1.clear();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
