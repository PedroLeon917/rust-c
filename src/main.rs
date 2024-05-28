pub enum LinkedList {} 

extern "C" {
    fn generateLinkedList() -> *mut LinkedList;
}

fn main() {
    let list;
    unsafe {
        list = generateLinkedList();
    }
}
