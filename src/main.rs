use std::collections::HashMap;

/// Represents the possible states of the Turing machine can be in.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum State {
    FindPlus,//moves to the + symbol
    GetLast,//gets the last digit to the left of the +
    AddOne,//remembers that a 1 has to be added to the end of the second number
    AddZero,//remembers that a 0 has to be added to the end of the second number
    AddDigitZero,//adds zero to the last non-altered digit of the second number(turns 0 to O and 1 to I)
    AddDigitOne,//adds one to the last non-altered digit of the second number
    Carry,//carries an additional one to the digits further left of the last addDigitOne if a 1 was encountered
    BackToStart,//moves back to start of the tape
    Halt,//We did it! :D
}

/// Represents the direction the head can move on the tape.
#[derive(Debug, Clone)]
enum Direction {
    Left,  // Move one step to the left.
    Right, // Move one step to the right.
}

/// Represents the Turing machine.
struct TuringMachine {
    tape: Vec<char>, // The tape holds symbols (e.g., '0', '1', '+', '_').
    head: usize,     // The current position of the head on the tape.
    state: State,    // The current state of the machine.
    rules: HashMap<(State, char), (char, Direction, State)>, // Transition rules.
}

impl TuringMachine {
    /// Creates a new Turing machine with the given tape and rules.
    fn new(tape: Vec<char>, rules: HashMap<(State, char), (char, Direction, State)>) -> Self {
        TuringMachine {
            tape,
            head: 0, // Start at the first position on the tape.
            state: State::FindPlus, // Initial state to locate the '+'.
            rules,
        }
    }

    /// Executes one step of the Turing machine.
    fn step(&mut self) {
        // Get the current symbol under the head.
        let current_symbol = self.tape[self.head];

        // Get the current state.
        let current_state = self.state.clone();

        // Look up the transition rule for the current state and symbol.
        if let Some(&(write, ref direction, ref next_state)) = self.rules.get(&(current_state, current_symbol)) {
            // Update the symbol under the head.
            self.tape[self.head] = write;

            // Move the head in the specified direction.
            match direction {
                Direction::Left => {
                    if self.head > 0 {
                        self.head -= 1;
                    } else {
                        // If at the start, expand the tape to the left.
                        self.tape.insert(0, '_');
                    }
                }
                Direction::Right => {
                    self.head += 1;
                    if self.head >= self.tape.len() {
                        // If at the end, expand the tape to the right.
                        self.tape.push('_');
                    }
                }
            }

            // Transition to the next state.
            self.state = next_state.clone();
        } else {
            // If no rule applies, halt the machine.
            self.state = State::Halt;
        }
    }

    /// Runs the Turing machine until it reaches the Halt state.
    fn run(&mut self) {
        while self.state != State::Halt {
            // Debugging output: Shows the tape, head position, and current state at each step.
            println!("Tape: {:?}, Head: {}, State: {:?}", self.tape, self.head, self.state);
            self.step();
        }
        // Final state and tape output after halting.
        println!("Final Tape: {:?}, Head: {}, State: {:?}", self.tape, self.head, self.state);
    }
}

fn main() {
    // Define the transition rules for the Turing machine.
    let mut rules = HashMap::new();
    

    /*
     * The rules below are defined as follows:
     * *IF* I am currently in a state X, looking at a cell of valy Y, then write Z in said cell,
     * Move in direction A, setting my state to state B. B could be X or any other state.
     * */

    //skips over the first empty cell
    rules.insert((State::FindPlus, '_'), ('_', Direction::Right, State::FindPlus));
    //keeps moving until we find a plus
    rules.insert((State::FindPlus, '1'), ('1', Direction::Right, State::FindPlus));
    rules.insert((State::FindPlus, '0'), ('0', Direction::Right, State::FindPlus));
    //turns back to get the last number
    rules.insert((State::FindPlus, '+'), ('+', Direction::Left, State::GetLast));
    //if previous number is 0, add zer0 to the second number, be sure to delete the number
    rules.insert((State::GetLast, '0'), ('+', Direction::Right, State::AddZero));
    //move all the way to the right, ignoring 1, 0 and + symbols
    rules.insert((State::AddZero, '1'), ('1', Direction::Right, State::AddZero));
    rules.insert((State::AddZero, '0'), ('0', Direction::Right, State::AddZero));
    rules.insert((State::AddZero, '+'), ('+', Direction::Right, State::AddZero));
    //stop at a _, I or O, and turn to the number on our left
    rules.insert((State::AddZero, 'I'), ('I', Direction::Left, State::AddDigitZero));
    rules.insert((State::AddZero, 'O'), ('O', Direction::Left, State::AddDigitZero));
    rules.insert((State::AddZero, '_'), ('_', Direction::Left, State::AddDigitZero));
    //adds a zero to the current digit, and moves all the way back to the start of the first number
    rules.insert((State::AddDigitZero, '1'), ('I', Direction::Left, State::BackToStart));
    rules.insert((State::AddDigitZero, '0'), ('O', Direction::Left, State::BackToStart));
    rules.insert((State::AddDigitZero, '+'), ('O', Direction::Left, State::BackToStart));

    //carries a 1 to the right and deletes the current digit
    rules.insert((State::GetLast, '1'), ('+', Direction::Right, State::AddOne));
    rules.insert((State::AddOne, '1'), ('1', Direction::Right, State::AddOne));
    rules.insert((State::AddOne, '0'), ('0', Direction::Right, State::AddOne));
    rules.insert((State::AddOne, '+'), ('+', Direction::Right, State::AddOne));
    rules.insert((State::AddOne, '_'), ('_', Direction::Left, State::AddDigitOne));
    rules.insert((State::AddOne, 'I'), ('I', Direction::Left, State::AddDigitOne));
    rules.insert((State::AddOne, 'O'), ('O', Direction::Left, State::AddDigitOne));
   
    //adds a zero to the last digit of the second number and moves back to plus
    rules.insert((State::AddDigitOne, '1'), ('O', Direction::Left, State::Carry));
    rules.insert((State::AddDigitOne, '0'), ('I', Direction::Left, State::BackToStart));
    rules.insert((State::AddDigitOne, '+'), ('I', Direction::Left, State::BackToStart));
    
    rules.insert((State::Carry, '0'), ('1', Direction::Left, State::BackToStart));
    rules.insert((State::Carry, '1'), ('0', Direction::Left, State::Carry));
    rules.insert((State::Carry, '+'), ('1', Direction::Left, State::BackToStart));

    //moves all the way back to the start of the tape
    rules.insert((State::BackToStart, '0'), ('0', Direction::Left, State::BackToStart));
    rules.insert((State::BackToStart, '1'), ('1', Direction::Left, State::BackToStart));
    rules.insert((State::BackToStart, '+'), ('+', Direction::Left, State::BackToStart));

    //once we reach the start, we pretend to start again
    rules.insert((State::BackToStart, '_'), ('_', Direction::Right, State::FindPlus));



    // Initialize the tape with two binary numbers and a '+' in between.
    let tape = vec!['_','1','0','1','0','0','1','1','0','1','1','+', '1', '0', '1','1', '_'];

    // Create the Turing machine with the tape and rules.
    let mut machine = TuringMachine::new(tape, rules);

    // Run the Turing machine.
    machine.run();
}
