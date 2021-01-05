#[derive(Debug)]
pub struct Config {
    pub peers: Vec<String>,
    pub address: String,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    peers: Vec<String>,
    address: String,
    name: String,
}

impl ConfigBuilder {
    pub fn builder() -> Self {
        ConfigBuilder::default()
    }

    pub fn peers(mut self, peers: Vec<String>) -> Self {
        self.peers = peers;
        self
    }

    pub fn address(mut self, address: String) -> Self {
        self.address = address;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn build(self) -> Config {
        Config {
            peers: self.peers,
            address: self.address,
            name: self.name,
        }
    }
}