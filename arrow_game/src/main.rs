use array_init::array_init;
use std::env;
use std::fs;
use std::io;

const BOARD_SIZE: usize = 17*17;

enum Direction{
    NONE,
    NORTH,
    EAST,
    SOUTH,
    WEST
}
enum Command{
    NONE,
    UP,
    RIGHT,
    DOWN,
    LEFT,
    FIRE
}

impl Default for Direction {
    fn default() -> Self { Direction::NONE }
}

enum Player{
    NONE,
    RED,
    GREEN,
    BLUE,
    YELLOW
}

impl Default for Player {
    fn default() -> Self { Player::NONE }
}

#[derive(Debug)]
enum TileId{
    NONE,
    PATH,
    CORNER,
    INTERSECTION,
    ORIGIN,
}

impl Default for TileId {
    fn default() -> Self { TileId::NONE }
}

#[derive(Default)]
struct Arrow{
    orientation: Direction,
    momentum: Direction
}

#[derive(Default)]
struct Ball{
    orientation: Direction,
    momentum: Direction
}

#[derive(Default)]
struct Tile{
    id: TileId,
    owner: Player,
    arrows: [Arrow; 4],
    balls: [Ball; 4],
}

fn load_board(board: &mut [Tile; BOARD_SIZE]){
    let game_map = fs::read_to_string("assets/level.board").unwrap();
    
    println!("With text:\n{}",game_map);

    for (row_idx, line) in game_map.lines().enumerate() {
        for (col_idx, char_str) in line.split(',').enumerate() {
            let index = row_idx * 17 + col_idx; // Calculate the 1D index
            let character = char_str.chars().next().unwrap();
            
            match character {
                'O' => board[index].id = TileId::ORIGIN,
                'P' => board[index].id = TileId::PATH,
                'I' => board[index].id = TileId::INTERSECTION,
                'C' => board[index].id = TileId::CORNER,
                _ => board[index].id = TileId::NONE,
            }

            let tile_id = &board[index].id;
            println!("[{row_idx}][{col_idx}] == {:?}", tile_id);
        }
    }
}

fn user_input(user_command: &mut Command) {

    while matches!(user_command, Command::NONE) {
    
        println!("Enter a command:");
        println!("1. UP");
        println!("2. RIGHT");
        println!("3. DOWN");
        println!("4. LEFT");
        println!("5. FIRE");
        
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                *user_command = parse_user_input(input);
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn parse_user_input(input: String) -> Command {
    let trimmed_input = input.trim();

    match trimmed_input {
        "1" => {
            println!("UP");
            Command::UP
        },
        "2" => {
            println!("RIGHT");
            Command::RIGHT
        },
        "3" => {
            println!("DOWN");
            Command::DOWN
        },
        "4" => {
            println!("LEFT");
            Command::LEFT
        },
        "5" => {
            println!("FIRE");
            Command::FIRE
        },
        _ => {
            println!("Invalid input: {}", trimmed_input);
            Command::NONE
        },
    }
}

fn update_board(board: &mut [Tile; BOARD_SIZE], user_command: &Command){
    // for idx in 0..(BOARD_SIZE-1) {
        
    // }
}

fn save_board(_board: &[Tile; BOARD_SIZE]){

}

fn game(){
    let mut board: [Tile; BOARD_SIZE] = array_init(|_| Tile::default());

    load_board(&mut board);

    let mut user_command = Command::NONE;
    user_input(&mut user_command);
    update_board(&mut board, &user_command);

    save_board(&board);
}

fn main(){
    /* Hardware Initialize */
    println!("Hardware Initialize");

    /* Software Initialize */
    println!("Software Initialize");

    /* Thread Initialize */
    println!("Thread Initialize");

    /* Application Code */
    println!("Application");
    game();

    /* Cleanup */
    println!("Cleanup");
}