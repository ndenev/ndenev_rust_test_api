
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    index: HashMap<String, Box<DataEntry>>,
}
impl Data {
    fn new() -> Data {
        Data { index: HashMap::new() }
    }
    fn insert(&mut self, v: Box<DataEntry>) {
        let k = v.id.clone();
        self.index.insert(k, v);
    }
    fn len(&self) -> usize {
        self.index.len()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: String,
    data: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataSize {
    data_len: usize,
}
