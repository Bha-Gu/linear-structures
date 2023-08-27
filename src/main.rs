mod double_linked;
mod linked;
use double_linked::DLList;
use linked::LinkedList;

fn main() {
    {
        println!("Linked List: ");
        let mut a = LinkedList::new();
        let b = a.copy_to_vec();
        println!("New: {b:?} {}", a.len());
        a.append(7);
        let b = a.copy_to_vec();
        println!("Append: {b:?} {}", a.len());
        a.prepend(6);
        let b = a.copy_to_vec();
        println!("Prepend: {b:?} {}", a.len());
        a.append(5);
        let b = a.copy_to_vec();
        println!("Append: {b:?} {}", a.len());
        a.insert_at(8, 2);
        let b = a.copy_to_vec();
        println!("Insert: {b:?} {}", a.len());
        let c = a.unprepend();
        let b = a.copy_to_vec();
        println!("Unprepend: {b:?} {c:?} {}", a.len());
        let c = a.remove_at(1);
        let b = a.copy_to_vec();
        println!("Remove: {b:?} {c:?} {}", a.len());
        let c = a.pop();
        let b = a.copy_to_vec();
        println!("Pop: {b:?} {c:?} {}", a.len());
        let c = a.pop();
        let b = a.copy_to_vec();
        println!("Pop: {b:?} {c:?} {}", a.len());
    }
    {
        println!("Double Linked List: ");
        let mut a = DLList::new();
        let b = a.copy_to_vec();
        println!("New: {b:?} {}", a.len());
        a.append(7);
        let b = a.copy_to_vec();
        println!("Append: {b:?} {}", a.len());
        a.prepend(6);
        let b = a.copy_to_vec();
        println!("Prepend: {b:?} {}", a.len());
        a.append(5);
        let b = a.copy_to_vec();
        println!("Append: {b:?} {}", a.len());
        a.insert_at(8, 2);
        let b = a.copy_to_vec();
        println!("Insert: {b:?} {}", a.len());
        let c = a.unprepend();
        let b = a.copy_to_vec();
        println!("Unprepend: {b:?} {c:?} {}", a.len());
        let c = a.remove_at(1);
        let b = a.copy_to_vec();
        println!("Remove: {b:?} {c:?} {}", a.len());
        let c = a.pop();
        let b = a.copy_to_vec();
        println!("Pop: {b:?} {c:?} {}", a.len());
        let c = a.pop();
        let b = a.copy_to_vec();
        println!("Pop: {b:?} {c:?} {}", a.len());
    }
}
