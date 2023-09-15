use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let initial_thrust = 100;
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let next_checkpoint_x = parse_input!(inputs[2], i32); // x position of the next check point
        let next_checkpoint_y = parse_input!(inputs[3], i32); // y position of the next check point
        let next_checkpoint_dist = parse_input!(inputs[4], i32); // distance to the next checkpoint
        let next_checkpoint_angle = parse_input!(inputs[5], i32); // angle between your pod orientation and the direction of the next checkpoint
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opponent_x = parse_input!(inputs[0], i32);
        let opponent_y = parse_input!(inputs[1], i32);

        let mut new_thrust = initial_thrust;
        let mut new_facing_x = next_checkpoint_x;
        let mut new_facing_y = next_checkpoint_y;
        let mut should_boost = false;
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // You have to output the target position
        // followed by the power (0 <= thrust <= 100)
        // i.e.: "x y thrust"
        if next_checkpoint_angle > 90 || next_checkpoint_angle < -90 {
            new_thrust = 10;
        }
        if next_checkpoint_angle > 160 || next_checkpoint_angle < -160 {
            new_thrust = 0;
        }
        if next_checkpoint_dist > 3000 && next_checkpoint_angle < 15 && next_checkpoint_angle > -15 {
            should_boost = true;
        }
        
        if should_boost {
            println!("{} {} BOOST", new_facing_x, new_facing_y)
        } else {
            println!("{} {} {}", new_facing_x, new_facing_y, new_thrust);
        } 
    }
}
