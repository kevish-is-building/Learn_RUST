fn main() {
    let _s = String::from("hello"); // Allocate Memo accordingly using ::from

    // For Stack Memo -> copy semantics is default
    let x = 5;
    let _y = x;

    // For Heap Memo -> Move semantics is default which is a same pointer to memo
    let s1 = String::from("hello");
    let _s2 = s1;

    // println!("hello with variable {s1} and {s2}")    // borrow of moved value: `s1` || s2 is having the OWNERSHIP

    let srt1 = String::from("value");
    let srt2 = srt1.clone();

    // takes_ownership(srt1);       // borrow of moved value: `srt1`

    // ownership fixes --->
    takes_ownership(srt1.clone());      
    let srt1 = takes_ownership(srt1);       // works as take and give ownership

    println!("value of srt1 -> `{srt1}` and srt2 -> `{srt2}`");

    let (srt1, srt1_len) = calculate_length(srt1);

    

}

fn takes_ownership(some_string: String) -> String { // some_string comes into scope
    println!("{some_string}");
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s,length)
}