mod mem_storage;
mod test_network;

#[derive(Debug,Clone)]
pub enum Action{
    Put(String,i32),
    Delete(String),
}