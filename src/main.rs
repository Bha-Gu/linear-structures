mod linked;
use linked::LinkedList;

fn main() {
    let mut a = LinkedList::new();
    let b = a.copy_to_vec();
    println!("New: {b:?}");
    a.append(7);
    let b = a.copy_to_vec();
    println!("Append: {b:?}");
    a.prepend(6);
    let b = a.copy_to_vec();
    println!("Prepend: {b:?}");
    a.append(5);
    let b = a.copy_to_vec();
    println!("Append: {b:?}");
    a.insert_at(8, 2);
    let b = a.copy_to_vec();
    println!("Insert: {b:?}");
    let c = a.unprepend();
    let b = a.copy_to_vec();
    println!("Unprepend: {b:?} {c:?}");
    let c = a.pop();
    let b = a.copy_to_vec();
    println!("Pop: {b:?} {c:?}");
    let c = a.pop();
    let b = a.copy_to_vec();
    println!("Pop: {b:?} {c:?}");
}
