// ============================================================================
//  CHAPTER 25 — RUST HASHMAP
//  `HashMap<K, V>` stores key-value pairs with O(1) average lookup.
// ============================================================================

use std::collections::HashMap;

fn main() {
    // ---- Creating a HashMap ---------------------------------------------
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    println!("scores: {:?}", scores);

    // From two iterators (zips them into key-value pairs):
    let teams = vec![String::from("Red"), String::from("Blue")];
    let initial_scores = vec![10, 50];
    let team_scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("team_scores: {:?}", team_scores);

    // ---- Accessing values -----------------------------------------------
    let alice_score = scores.get(&String::from("Alice"));   // Option<&V>
    println!("Alice's score: {:?}", alice_score);

    let missing = scores.get(&String::from("Charlie"));     // None
    println!("Charlie's score: {:?}", missing);

    // ---- Iterating -------------------------------------------------------
    for (name, score) in &scores {
        println!("{} -> {}", name, score);
    }

    // ---- Inserting and updating -----------------------------------------
    // Insert overwrites the existing value:
    scores.insert(String::from("Alice"), 15);   // was 10, now 15
    println!("after overwrite: {:?}", scores);

    // `entry` + `or_insert` — insert only if key is absent:
    scores.entry(String::from("Alice")).or_insert(99);  // already present, no change
    scores.entry(String::from("Charlie")).or_insert(99); // inserted
    println!("after entry/or_insert: {:?}", scores);

    // Updating based on the old value:
    let text = "hello world wonderful world";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;                              // dereference and increment
    }
    println!("word_count: {:?}", word_count);

    // ---- Removing --------------------------------------------------------
    let removed = scores.remove(&String::from("Charlie"));   // Option<V>
    println!("removed Charlie: {:?}, remaining: {:?}", removed, scores);

    // ---- Length and emptiness -------------------------------------------
    println!("len = {}, empty? {}", scores.len(), scores.is_empty());

    // ---- Contains key ----------------------------------------------------
    println!("has Alice? {}", scores.contains_key(&String::from("Alice")));

    // ---- Ownership notes -------------------------------------------------
    // Types that are Copy (i32, &str, ...) are copied into the map.
    // Types that are not (String, Vec, ...) are MOVED into the map.
    let key = String::from("owned");
    let val = vec![1, 2, 3];
    let mut owned_map: HashMap<String, Vec<i32>> = HashMap::new();
    owned_map.insert(key, val);
    // println!("{}", key);   // ERROR: key was moved
    // println!("{:?}", val); // ERROR: val was moved

    // ---- Choosing key types ----------------------------------------------
    // HashMap requires K: Eq + Hash. Most standard types implement both.
    // For your own types, derive Eq + Hash:
    //   #[derive(Debug, PartialEq, Eq, Hash)]
    //   struct Point { x: i32, y: i32 }
    let mut point_map: HashMap<(i32, i32), &str> = HashMap::new();
    point_map.insert((0, 0), "origin");
    point_map.insert((1, 2), "A");
    println!("point_map: {:?}", point_map);

    // ---- BTreeMap alternative (sorted) ----------------------------------
    // HashMap has no defined iteration order. If you need sorted keys,
    // use std::collections::BTreeMap instead.
    use std::collections::BTreeMap;
    let mut sorted: BTreeMap<i32, &str> = BTreeMap::new();
    sorted.insert(3, "three");
    sorted.insert(1, "one");
    sorted.insert(2, "two");
    println!("BTreeMap (sorted): {:?}", sorted);   // {1: "one", 2: "two", 3: "three"}
}
