fn next(current_state: char, read_symbol: i8) -> (char, i8, i8) {
    let mut next_state: char = 'Y';
    let mut write_symbol: i8 = 2;
    let mut shift: i8 = 0;
    
    if current_state == 'A' {
        if read_symbol == 0 {
            next_state = 'B';
            write_symbol = 1;
            shift = 1;
        } else {
            next_state = 'B';
            write_symbol = 1;
            shift = -1;
        }

    } else if current_state == 'B' {
        if read_symbol == 0 {
            next_state = 'A';
            write_symbol = 1;
            shift = -1;
        } else {
            next_state = 'C';
            write_symbol = 0;
            shift = -1;
        }

    } else if current_state == 'C' {
        if read_symbol == 0 {
            next_state = 'X'; // HALT
            write_symbol = 1;
            shift = 1;
        } else {
            next_state = 'D'; 
            write_symbol = 1;
            shift = -1;
        } 

    } else if current_state == 'D' {
        if read_symbol == 0 {
            next_state = 'D';
            write_symbol = 1;
            shift = 1;
        } else {
            next_state = 'A'; 
            write_symbol = 0;
            shift = 1;
        } 
} 

    return (next_state, write_symbol, shift);
}

fn main() {
    const size: usize = 256; // change this to 2^16 later 
    let mut tape: [i8; size] = [0; size]; // creating a tape

    let mut tape_pos: i32 = size as i32 /2; 
    let mut state: char = 'A';
    
    while state != 'X' {
        let change: (char, i8, i8);
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
