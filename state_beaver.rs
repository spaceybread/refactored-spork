fn next_4_2(current_state: i32, read_symbol: i8) -> (i32, i8, i8) {
    // 4 states, 2 symbols
    // (next state, write symbol, shift)
    let A: i32 = 0;
    let B: i32 = 1; 
    let C: i32 = 2; 
    let D: i32 = 3; 
    let X: i32 = -1;

    let L: i8 = -1;
    let R: i8 = 1;

    // (next state, write symbol, shift)
    let state_table: Vec<Vec<(i32, i8, i8)>> = vec![
        vec![(B, 1, R), (A, 1, L), (X, 1, R), (D, 1, R)],
        vec![(B, 1, L), (C, 0, L), (D, 1, L), (A, 0, R)],
    ];

    return state_table[read_symbol as usize][current_state as usize];
}

fn next_3_2(current_state: i32, read_symbol: i8) -> (i32, i8, i8) {
    // 3 states, 2 symbols
    // (next state, write symbol, shift)
    let A: i32 = 0;
    let B: i32 = 1; 
    let C: i32 = 2; 
    let X: i32 = -1;

    let L: i8 = -1;
    let R: i8 = 1;

    // (next state, write symbol, shift)
    let state_table: Vec<Vec<(i32, i8, i8)>> = vec![
        vec![(B, 1, R), (C, 0, R), (C, 1, L)],
        vec![(X, 1, R), (B, 1, R), (A, 1, L)],
    ];

    return state_table[read_symbol as usize][current_state as usize];
}

fn main() {
    const size: usize = 64; // change this to 2^16 later if required
    let mut tape: [i8; size] = [0; size]; // creating a tape

    let mut tape_pos: i32 = size as i32 /2; 
    let mut state: i32 = 0;
    
    while state != -1 {
        let change: (i32, i8, i8);
        change = next_4_2(state, tape[tape_pos as usize]); // find the next state
        
        tape[tape_pos as usize] = change.1;
        tape_pos += change.2 as i32; 
        state = change.0;

        println!("{:?}", tape);
    } 

    // if this halts
    let mut count: i32 = 0;
    for n in 1..size as i32 {
        if tape[n as usize] == 1 {
             count += 1; 
        }
    }

    println!("{count}");

}
