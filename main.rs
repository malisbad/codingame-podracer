use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // set initial values for the game run
    let mut round_counter = 0;
    let initial_thrust = 100;
    let mut opponent_prev_x: i32 = 0;
    let mut opponent_prev_y: i32 = 0;
    let mut self_prev_x: i32 = 0;
    let mut self_prev_y: i32 = 0;

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

        if (round_counter == 0) {
            self_prev_x = x;
            self_prev_y = y;
            opponent_prev_x = opponent_x;
            opponent_prev_y = opponent_y;
        }

        // mutable values based on calculations for final commands
        let mut new_thrust = initial_thrust;
        let mut new_facing_x = next_checkpoint_x;
        let mut new_facing_y = next_checkpoint_y;
        let mut should_boost = false;
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        let own_velocity = determine_velocity(self_prev_x, self_prev_y, x, y);
        let opponent_velocity = determine_velocity(opponent_prev_x, opponent_prev_y, opponent_x, opponent_y);
        eprintln!("Own velocity: {}", own_velocity.total);
        eprintln!("Opponent velocity: {}", opponent_velocity.total);
        // TODO there is some equation that has the best power/velocity/turning radius, ask GPT
        // TODO there is some seed we can do to optimize for each course
        // TODO add a simple NN to optimize turning radius for the course after each run
        /* 
            TODO use pursuit equation if we're behind to ram an opponent off course. 
            This is probably best done when they are at a low velocity with a similar facing to us that
            isn't in line with the target destination. This should result in maximum loss of ground by the
            opponent. Even better if we can knock them out just before the move over a target destination.
        */

        /*
            If the turning angle is greater than hard left/right, reduce velocity to decrease turning radius

            TODO refactor to have an optimal turning radius calculation
        */
        if next_checkpoint_angle > 90 || next_checkpoint_angle < -90 {
            new_thrust = 10;
            eprintln!("Next destination is a {} degrees at {} units, cutting thrust to {}", next_checkpoint_angle, next_checkpoint_dist, new_thrust);
        }

        /* 
            if the angle is extremely high (0 being straight ahead), kill velcoity
            to maximize turning speed without drift

            TODO add calcuation for best velocity/turning radius
        */
        if next_checkpoint_angle > 160 || next_checkpoint_angle < -160 {
            new_thrust = 0;
            eprintln!("Next destimation is at {} degrees at {} units, cutting thrust to {}", next_checkpoint_angle, next_checkpoint_dist, new_thrust);
        }

        // TODO boost is used too sparingly, should make conditions a little less painful
        if next_checkpoint_dist > 3000 && next_checkpoint_angle < 15 && next_checkpoint_angle > -15 {
            should_boost = true;
            eprintln!("Next destimation is at {} degrees at {} units, boosting", next_checkpoint_angle, next_checkpoint_dist);
        }
        
        if should_boost {
            println!("{} {} BOOST", new_facing_x, new_facing_y)
        } else {
            println!("{} {} {}", new_facing_x, new_facing_y, new_thrust);
        }
        round_counter = round_counter + 1;
    }
}

/**
    Calculate the intercept of the opponent give our position, their position, our velocity, and their velocity
*/
fn pursuit_equation(target_position: (i32, i32), pursuer_speed: i32, target_speed: i32, initial_pursuer_position: (i32, i32)) -> (i32, i32) {
    // Calculate the distance between the target and the pursuer
    let dx = target_position.0 - initial_pursuer_position.0;
    let dy = target_position.1 - initial_pursuer_position.1;
    let distance_squared = dx*dx + dy*dy;

    // Calculate the relative speed
    let relative_speed = pursuer_speed - target_speed;

    // Calculate the time to intercept (approximated to the nearest integer)
    let time_to_intercept = (distance_squared as f64).sqrt() as i32 / relative_speed;

    // Predict the target's position at the time of intercept
    let predicted_target_position = (
        target_position.0 + target_speed * time_to_intercept,
        target_position.1 + target_speed * time_to_intercept
    );

    // Return the predicted target position
    predicted_target_position
}

/**
    Calculate the optimal turning radius for each destination after the course is mapped
*/
fn calculate_optimal_turning_radius() -> (i32, i32) {
    (0, 0)
}

struct OpponentVelocity {
    x: i32,
    y: i32,
    total: i32
}
/**
    Calcuates the opponent's current velocity based on their x, y movement. Returns a tuple of
    their x velocity, y velocity, and speed
*/
fn determine_velocity(x1: i32, y1: i32, x2: i32, y2: i32) -> OpponentVelocity {
    let a = f64::sqrt((x1 - x2) as f64).round().abs() as i32;
    let b = f64::sqrt((y1 - y2) as f64).round().abs() as i32;
    let total = a + b;

    OpponentVelocity {
        x: a,
        y: b,
        total: total,
    }
}