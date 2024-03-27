fn next(current_state: i32, read_symbol: i8) -> (i32, i8, i8) {
    
    // (next state, write symbol, shift)
    // A: 0, B: 1, C: 2, D: 3, HALT: -1
    // L: -1, R: 1

    let state_table: Vec<Vec<(i32, i8, i8)>> = vec![
        vec![(1, 1, 1), (0, 1, -1), (-1, 1, 1), (3, 1, 1)],
        vec![(1, 1, -1), (2, 0, -1), (3, 1, -1), (0, 0, 1)],
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
        change = next(state, tape[tape_pos as usize]); // find the next state
        
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