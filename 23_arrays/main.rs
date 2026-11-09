// ============================================================================
//  CHAPTER 22 — RUST ARRAYS
//  Fixed-size, same-type, stack-allocated sequences.
// ============================================================================

fn main() {
    // ---- Creating arrays -------------------------------------------------
    let nums: [i32; 5] = [1, 2, 3, 4, 5];      // 5 i32s
    let zeros = [0; 10];                        // 10 zeros (shorthand init)
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    println!("nums: {:?}", nums);
    println!("zeros: {:?}", zeros);
    println!("months: {:?}", months);

    // ---- Accessing elements ---------------------------------------------
    let first = nums[0];          // 1
    let last  = nums[4];          // 5
    println!("first = {}, last = {}", first, last);

    // ---- Bounds checking (runtime) --------------------------------------
    // Rust checks array indices at runtime. Out-of-bounds access PANICS.
    // let oob = nums[10];   // would panic at runtime: index out of bounds

    // ---- Length ----------------------------------------------------------
    println!("nums length: {}", nums.len());
    println!("nums is empty? {}", nums.is_empty());

    // ---- Iterating -------------------------------------------------------
    for n in nums.iter() {
        print!("{} ", n);          // 1 2 3 4 5
    }
    println!();

    for (i, n) in nums.iter().enumerate() {
        print!("[{}]={} ", i, n);
    }
    println!();

    // ---- Mutable arrays --------------------------------------------------
    let mut scores = [0; 3];
    scores[0] = 10;
    scores[1] = 20;
    scores[2] = 30;
    println!("scores: {:?}", scores);

    // ---- Slices — borrowed view of an array -----------------------------
    let slice: &[i32] = &nums[1..4];   // [2, 3, 4]
    println!("slice: {:?}", slice);

    // ---- Arrays as function parameters ----------------------------------
    // Pass arrays by reference (slice) to avoid copying:
    let sum = sum_array(&nums);
    println!("sum of nums = {}", sum);

    // Or by value (copies the array — small arrays OK, large ones wasteful):
    print_array(months);

    // ---- Multi-dimensional arrays ---------------------------------------
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("matrix: {:?}", matrix);
    println!("matrix[1][2] = {}", matrix[1][2]);   // 6

    // ---- Array length is part of the type -------------------------------
    // [i32; 5] and [i32; 6] are DIFFERENT types.
    let a3: [i32; 3] = [1, 2, 3];
    let a4: [i32; 4] = [1, 2, 3, 4];
    // sum_array(a4);   // ERROR: expected [i32; 5], found [i32; 4]
    println!("a3 = {:?}, a4 = {:?}", a3, a4);

    // ---- Convert array to Vec -------------------------------------------
    let vec_from_arr: Vec<i32> = nums.to_vec();
    println!("as Vec: {:?}", vec_from_arr);

    // ---- Const generics: function that works for any array size ---------
    let s5 = sum_any_size(&[1, 2, 3, 4, 5]);
    let s3 = sum_any_size(&[10, 20, 30]);
    println!("sum_any_size: 5-elem = {}, 3-elem = {}", s5, s3);
}

// Pass by slice — works for arrays of any size.
fn sum_array(arr: &[i32; 5]) -> i32 {
    let mut total = 0;
    for &n in arr {
        total += n;
    }
    total
}

// Better: take a slice, which works for any array size:
fn sum_any_size(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn print_array(arr: [&str; 12]) {
    for (i, m) in arr.iter().enumerate() {
        print!("{}={} ", i + 1, m);
    }
    println!();
}
