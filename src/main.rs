mod double_linked;
mod linked;
mod queue;
mod stack;
use double_linked::DLList;
use linked::LinkedList;
use queue::Queue;
use stack::Stack;

fn main() {
    //Linked List Test
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
    //Double Linked List Test
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
    //Queue Test
    {
        let mut a = Queue::new();
        a.enqueue(5);
        println!("{:?} {}", a.peek(), a.len());
        println!("{:?} {}", a.deque(), a.len());
        println!("{:?} {}", a.deque(), a.len());
    }
    {
        let mut a = Stack::new();
        a.push(5);
        println!("{:?} {}", a.peek(), a.len());
        println!("{:?} {}", a.pop(), a.len());
        println!("{:?} {}", a.pop(), a.len());
    }
}
