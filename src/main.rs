mod linked;
use linked::LinkedList;

fn main() {
    let mut a = LinkedList::new();
    a.append(7);
    a.prepend(6);
    a.append(5);
    a.insert_at(8, 4);
    let b = a.copy_to_vec();
    println!("{b:?}");
}
