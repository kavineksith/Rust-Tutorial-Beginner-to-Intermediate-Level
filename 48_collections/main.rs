// ============================================================================
//  CHAPTER 41 — RUST COLLECTIONS (in depth)
//  Beyond Vec and HashMap: BTreeMap, BTreeSet, HashSet, VecDeque,
//  LinkedList, BinaryHeap. When to use each.
// ============================================================================

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    println!("=== Collections (in depth) ===\n");

    // ======================================================================
    //  1. HashMap<K, V> — O(1) average lookup, UNORDERED
    // ======================================================================
    //  Reviewed in chapter 25 — quick recap:
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);
    scores.insert("Carol", 30);
    println!("HashMap: {:?}", scores);
    println!("Alice's score: {:?}", scores.get("Alice"));

    // ======================================================================
    //  2. BTreeMap<K, V> — O(log n) lookup, SORTED by key
    // ======================================================================
    //  Iteration is in sorted key order. Use when you need ordered access
    //  or range queries. Key type must implement Ord.
    let mut btree: BTreeMap<&str, i32> = BTreeMap::new();
    btree.insert("zebra", 1);
    btree.insert("apple", 2);
    btree.insert("mango", 3);
    btree.insert("banana", 4);
    println!("\nBTreeMap (always sorted):");
    for (k, v) in &btree {
        println!("  {} -> {}", k, v);   // apple, banana, mango, zebra
    }

    // Range queries — BTreeMap-specific:
    let middle: BTreeMap<&str, &i32> = btree.range("banana".."mango").map(|(k, v)| (*k, v)).collect();
    println!("  range banana..mango: {:?}", middle);

    // ======================================================================
    //  3. HashSet<T> — unordered set of unique values, O(1) lookup
    // ======================================================================
    //  Like HashMap but only keys (no values).
    let mut fruits: HashSet<&str> = HashSet::new();
    fruits.insert("apple");
    fruits.insert("banana");
    fruits.insert("cherry");
    fruits.insert("apple");   // ignored — already present
    println!("\nHashSet: {:?}, len = {}", fruits, fruits.len());
    println!("  contains 'banana'? {}", fruits.contains(&"banana"));

    // Set operations:
    let tropical: HashSet<&str> = ["banana", "mango", "pineapple"].iter().copied().collect();
    println!("  fruits & tropical (intersection): {:?}", fruits.intersection(&tropical).collect::<Vec<_>>());
    println!("  fruits | tropical (union):          {:?}", fruits.union(&tropical).collect::<Vec<_>>());
    println!("  fruits - tropical (difference):     {:?}", fruits.difference(&tropical).collect::<Vec<_>>());
    println!("  symmetric difference:               {:?}", fruits.symmetric_difference(&tropical).collect::<Vec<_>>());

    // ======================================================================
    //  4. BTreeSet<T> — sorted set, O(log n) lookup
    // ======================================================================
    let mut sorted: BTreeSet<i32> = BTreeSet::new();
    sorted.insert(5);
    sorted.insert(1);
    sorted.insert(3);
    sorted.insert(2);
    sorted.insert(4);
    println!("\nBTreeSet (sorted): {:?}", sorted.iter().collect::<Vec<_>>());
    println!("  first: {:?}, last: {:?}", sorted.iter().next(), sorted.iter().last());

    // ======================================================================
    //  5. VecDeque<T> — double-ended queue, O(1) push/pop at both ends
    // ======================================================================
    //  Growable ring buffer. Use when you need to add/remove from BOTH the
    //  front and the back efficiently (Vec is O(n) at the front).
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(2);
    deque.push_back(3);
    deque.push_front(1);
    deque.push_front(0);
    println!("\nVecDeque: {:?}", deque);   // [0, 1, 2, 3]
    println!("  pop_front = {:?}, deque = {:?}", deque.pop_front(), deque);
    println!("  pop_back  = {:?}, deque = {:?}", deque.pop_back(), deque);

    // ======================================================================
    //  6. LinkedList<T> — rarely the right choice in Rust
    // ======================================================================
    //  Doubly linked list. O(1) insert/remove at any position IF you have
    //  a cursor pointing there. But getting to that position is O(n).
    //
    //  In Rust, LinkedList is almost NEVER what you want. Prefer:
    //    * Vec / VecDeque for ordered storage
    //    * HashMap / HashSet for lookup
    //    * Box<dyn Trait> or Rc<RefCell<T>> for graphs
    //
    //  Shown here for completeness.
    let mut list: LinkedList<&str> = LinkedList::new();
    list.push_back("a");
    list.push_back("b");
    list.push_front("first");
    println!("\nLinkedList: {:?}", list);

    // ======================================================================
    //  7. BinaryHeap<T> — priority queue, max-heap by default
    // ======================================================================
    //  O(1) peek, O(log n) push/pop. Always returns the LARGEST element.
    let mut heap = BinaryHeap::new();
    heap.push(5);
    heap.push(1);
    heap.push(10);
    heap.push(3);
    println!("\nBinaryHeap: {:?}", heap);
    println!("  peek (max): {:?}", heap.peek());
    println!("  pop order:");
    while let Some(n) = heap.pop() {
        print!("{} ", n);          // 10 5 3 1 (descending)
    }
    println!();

    // Min-heap: use Reverse<T> or std::cmp::Reverse
    use std::cmp::Reverse;
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(10));
    println!("  min-heap pop order:");
    while let Some(Reverse(n)) = min_heap.pop() {
        print!("{} ", n);          // 1 5 10 (ascending)
    }
    println!();

    // ======================================================================
    //  SUMMARY TABLE
    // ========================================================================
    //
    //  Collection     | Lookup    | Ordered? | Use when...
    //  ---------------+-----------+----------+--------------------------------
    //  Vec<T>         | O(1) idx  | insert   | default list; need indexing
    //  VecDeque<T>    | O(1) ends | insert   | push/pop both ends (queue/deque)
    //  LinkedList<T>  | O(n)      | insert   | almost never — prefer Vec/VecDeque
    //  HashMap<K,V>   | O(1) avg  | no       | key-value lookup, no order needed
    //  BTreeMap<K,V>  | O(log n)  | yes      | sorted keys / range queries
    //  HashSet<T>     | O(1) avg  | no       | unique values, fast membership
    //  BTreeSet<T>    | O(log n)  | yes      | unique values, sorted iteration
    //  BinaryHeap<T>  | O(log n)  | heap     | priority queue (max first)
    //
    //  Default choices:
    //    List      -> Vec<T>
    //    Map       -> HashMap<K, V>  (or BTreeMap if you need order)
    //    Set       -> HashSet<T>     (or BTreeSet if you need order)
    //    Queue     -> VecDeque<T>
    //    Priority  -> BinaryHeap<T>
    // ========================================================================
}

