use std::ptr::NonNull;

#[repr(C)]
pub struct Position {
    x: i32,
    y: i32,
}

// Since `LinkedList` are opaque types (we don't need their
// internal structure in Rust), we represent them as empty enums.
// These types will only be manipulated through pointers.
pub enum LinkedList {}

extern "C" {
    fn generateLinkedList() -> *mut LinkedList;
    fn pushBack(linkedList: *mut LinkedList, x: i32, y: i32);
    fn printList(linkedList: *mut LinkedList);
    fn freeList(linkedList: *mut LinkedList);
    fn get(LinkedList: *mut LinkedList, index: i32) -> Position;
    fn searchList(LinkedList: *mut LinkedList, x: i32, y: i32) -> *mut Position;
}
fn main() {
    let list;
    unsafe {
        list = generateLinkedList();
        pushBack(list, 5, 10);
        pushBack(list, 11, 12);
        pushBack(list, 16, 40);
        pushBack(list, 40, 100);
        printList(list);

        let pos = get(list, 2);
        println!("pos: ({}, {})", pos.x, pos.y);

        let pos2 = searchList(list, 16, 40);

        // if !pos2.is_null() {
        //     println!("pos: ({}, {})", (*pos2).x, (*pos2).y);
        // } else {
        //     println!("Data is not found");
        // }

        let pos_option = NonNull::new(pos2);
        if let Some(data) = pos_option {
            println!("pos: ({}, {})", data.as_ref().x, data.as_ref().y);
        } else {
            println!("Data is not found");
        }

        freeList(list);
    }
}