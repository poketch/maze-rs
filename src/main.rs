use maze_rs::{maze_plot, maze_gen::Maze};


fn main() {
    // maze_plot::draw();
    let maze: Maze<20, 20> = maze_rs::maze_gen::Maze::new();
    println!("{}", maze);
}
