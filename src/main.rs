use std::io::Write;

fn main() {
    // init data structure
    // 0 -> empty; 1 -> X; 2 -> O
    let mut game_state = [0; 9];
    let mut current_player = 2;

    // game loop
    loop {
        let mut player = String::new();
        if current_player == 1 {
            player = "X".to_string();
        } else if current_player == 2 {
            player = "O".to_string();
        } else {
            println!("Wrong player. This should not happen.");
        }
        println!("Current player: {}", player);

        print_game_state(game_state);
        
        // get user input
        print!("Move (positions 0-8): ");
        std::io::stdout().flush().expect("Flush error.");

        let mut current_move_input = String::new();
        std::io::stdin()
            .read_line(&mut current_move_input)
            .expect("Failed to read line");
        let current_move: usize = match current_move_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input. Try again…\n");
                continue;
            },
        };

        if current_move > 8 || game_state[current_move] != 0 {
            println!("Wrong input. Try again…\n");
            continue;
        }
        println!("input: {}", current_move);

        game_state[current_move] = current_player;

        // switch players
        switch_current_player(&mut current_player);

        // is the game over
        let game_over_value = is_game_over(game_state);
        if game_over_value != 0 {
            print_game_state(game_state);
            println!("Game over!");

            if game_over_value == 3 {
                println!("It's a tie!");
            } else {
                println!("Player {} won!", player);
            }
            break;
        }
    }
}

fn print_game_state(game_state: [u32; 9]) {
    for i in 0..3 {
        for j in 0..3 {
            let current_game_state_in_cell = game_state[(i*3) + j];
            let mut state = String::new();
            if current_game_state_in_cell == 1 {
                state = "X".to_string();
            } else if current_game_state_in_cell == 2 {
                state = "O".to_string();
            } else if current_game_state_in_cell == 0 {
                state = " ".to_string();
            } else {
                println!("Wrong state. This should not happen.");
            }
            print!(" {}", state);
            if j < 2 {
                print!(" |");
            }
        }
        print!("\n");
        if i < 2 {
            print!("---+---+---\n");
        }
    }
}

fn switch_current_player(current_player: &mut u32) {
    if *current_player == 1 {
        *current_player = 2;
    } else if *current_player == 2 {
        *current_player = 1;
    } else {
        println!("Invalid player");
    }
}

fn is_game_over(game_state: [u32; 9]) -> u32 {
    // check horizontal
    if game_state[0] == game_state[1] && game_state[0] == game_state[2] && game_state[0] != 0 {
        return game_state[0];
    } else if game_state[3] == game_state[4] && game_state[3] == game_state[5] && game_state[3] != 0 {
        return game_state[3];
    } else if game_state[6] == game_state[7] && game_state[6] == game_state[8] && game_state[6] != 0 {
        return game_state[5];
    }
    // check vertical
    else if game_state[0] == game_state[3] && game_state[0] == game_state[6] && game_state[0] != 0 {
        return game_state[0];
    } else if game_state[1] == game_state[4] && game_state[1] == game_state[7] && game_state[1] != 0 {
        return game_state[1];
    } else if game_state[2] == game_state[5] && game_state[2] == game_state[8] && game_state[2] != 0 {
        return game_state[2];
    }
    // check diagonal
    else if game_state[0] == game_state[4] && game_state[0] == game_state[8] && game_state[0] != 0 {
        return game_state[0];
    } else if game_state[2] == game_state[4] && game_state[2] == game_state[6] && game_state[2] != 0 {
        return game_state[2];
    }
    // check if tie
    else if game_state[0] != 0 && game_state[1] != 0 && game_state[2] != 0 &&
             game_state[3] != 0 && game_state[4] != 0 && game_state[5] != 0 &&
                 game_state[6] != 0 && game_state[7] != 0 && game_state[8] != 0 {
                     return 3;
                 }
    return 0;
}
