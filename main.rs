use std::io;
use std::f64::consts::PI;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Velocity {
    x: i32,
    y: i32,
    speed: i32
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
struct Position {
    x: i32,
    y: i32
}

struct Player {
    prev_pos: Position,
    current_pos: Position,
    thrust: i32,
    velocity: Velocity
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // TODO create a GameState struct that holds prev_state
    let mut opponent = Player::new();
    let mut own = Player::new();
    // set initial values for the game run
    let mut round_counter = 0;
    let mut course_complete = false;
    let mut course: Vec<Position> = Vec::new(); // vector for holding checkpoints
    let mut prev_checkpoint = Position { x: 0, y: 0}; // state for previous checkpoint
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

        let next_checkpoint = Position { x: next_checkpoint_x, y: next_checkpoint_y };

        // Set the previous position to be whatever comes from the game on the first round
        if round_counter == 0 {
            own.update_prev_pos(own.current_pos.x, own.current_pos.y);
            own.update_prev_pos(opponent.current_pos.x, opponent.current_pos.y);
        }

        // populate the course checkpoints for optimization
        if course_complete == false && !course.is_empty() &&prev_checkpoint != next_checkpoint && course.contains(&next_checkpoint) {
            course_complete = true;
        } else if course_complete == false && prev_checkpoint != next_checkpoint {
            course.push(next_checkpoint.clone());
            prev_checkpoint = next_checkpoint;
        }
        
        own.update_current_pos(x, y);
        opponent.update_current_pos(opponent_x, opponent_y);
        // update Player properties from the differences between previous round and this one
        let new_self_velocity = determine_velocity(&own.current_pos, &own.prev_pos);
        let new_opponent_velocity = determine_velocity(&opponent.current_pos, &opponent.prev_pos);
        own.update_velocity(new_self_velocity);
        opponent.update_velocity(new_opponent_velocity);

        // mutable values based on calculations for final commands
        let mut new_thrust = own.thrust;
        let mut new_facing_x = next_checkpoint_x;
        let mut new_facing_y = next_checkpoint_y;
        let mut should_boost = false;
        let mut boost_remaining: i32 = 1;
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        eprintln!("Own velocity: {}", own.velocity.speed);
        eprintln!("Opponent velocity: {}", opponent.velocity.speed);
        // TODO there is some equation that has the best power/velocity/turning radius, ask GPT
        // TODO there is some seed we can do to optimize for each course
        // TODO add a simple NN to optimize turning radius for the course after each run
        /* 
            TODO use pursuit equation if we're behind to ram an opponent off course. 
            This is probably best done when they are at a low velocity with a similar facing to us that
            isn't in line with the target destination. This should result in maximum loss of ground by the
            opponent. Even better if we can knock them out just before the move over a target destination.
        */
        let target_destination = Position { x: next_checkpoint_x, y: next_checkpoint_y}; //for testing, will assume the same target
        let intercept_target = pursuit_equation(&opponent.current_pos, &target_destination, &own.velocity.speed, &opponent.velocity.speed, &own.current_pos);
        match intercept_target {
            None => eprintln!("No solution found for intercept"),
            Some(intercept) => eprintln!("Intercept target: {}, {}", intercept.x, intercept.y),
        }

        let distance_between_players = calculate_distance(&own.current_pos, &opponent.current_pos);
        if distance_between_players < 850 {
            eprintln!("COLLISION IMMINENT")
        }
        eprintln!("Distance between players: {}", distance_between_players);
        // TODO Determine if the opponent is going after the same checkpoint or not

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
        
        // TODO Boosting in to an opponent, hitting the shield and then rocketing them off course
        if boost_remaining > 0 && next_checkpoint_dist > 4000 && next_checkpoint_angle < 15 && next_checkpoint_angle > -15 {
            should_boost = true;
            boost_remaining = boost_remaining - 1;
            eprintln!("Next destimation is at {} degrees at {} units, boosting", next_checkpoint_angle, next_checkpoint_dist);
        }
        
        if should_boost {
            println!("{} {} BOOST", new_facing_x, new_facing_y);
            should_boost = false;
        } else {
            println!("{} {} {}", new_facing_x, new_facing_y, new_thrust);
        }
        round_counter = round_counter + 1;
    }
}

// Factory for players
impl Player {
    fn new() -> Player {
        Player {
            prev_pos: Position {
                x: 0,
                y: 0
            },
            current_pos: Position {
                x: 0,
                y: 0
            },
            thrust: 100,
            velocity: Velocity {
                x: 0,
                y: 0,
                speed: 0
            }
        }
    }

    // Update methods
    fn update_prev_pos(&mut self, x: i32, y: i32) {
        self.prev_pos.x = x;
        self.prev_pos.y = y;
    }
    
    fn update_current_pos(&mut self, x: i32, y: i32) {
        self.current_pos.x = x;
        self.current_pos.y = y;
    }

    fn update_thrust(&mut self, thrust: i32) {
        self.thrust = thrust;
    }

    fn update_velocity(&mut self, new_velocity: Velocity) {
        self.velocity = new_velocity;
    }
}


/**
    Calculates the distance between two points on the map
*/
fn calculate_distance(current_position: &Position, prev_position: &Position) -> i32 {
    let dx = current_position.x - prev_position.x;
    let dy = current_position.y - prev_position.y;
    let distance_squared = dx*dx + dy*dy;
    let distance = (distance_squared as f64).sqrt() as i32;
    distance
}

/**
    Calculate the intercept of the opponent give our position, their position, our velocity, and their velocity
*/
fn pursuit_equation(target_position: &Position, target_destination: &Position, pursuer_speed: &i32, target_speed: &i32, initial_pursuer_position: &Position) -> Option<Position> {
    // constants
    let max_velocity = 195; // determined through trial and error
    let min_intercept_distance = 800;
    let min_intercept_angle: f64 = 20.;

    // If the target's speed is higher than the maximum velocity, the pursuer will never catch the target
    if target_speed > &max_velocity {
        return None;
    }

    // Calculate the target's path vector
    let dx_path = target_destination.x - target_position.x;
    let dy_path = target_destination.y - target_position.y;

    // Calculate the distance to the destination
    let distance_to_destination = ((dx_path*dx_path + dy_path*dy_path) as f64).sqrt();
    
    // If the target is within the minimum intercept distance of the destination, the pursuer cannot intercept
    if distance_to_destination < min_intercept_distance as f64 {
        return None;
    }

    // Calculate the time to destination
    let time_to_destination = (distance_to_destination / *target_speed as f64).round() as i32;

    // Predict the target's position at the time of intercept
    let predicted_target_position = Position {
        x: target_position.x + dx_path * time_to_destination / target_speed,
        y: target_position.y + dy_path * time_to_destination / target_speed,
    };

    // Calculate the distance from the pursuer to the predicted position
    let dx_pursuit = predicted_target_position.x - initial_pursuer_position.x;
    let dy_pursuit = predicted_target_position.y - initial_pursuer_position.y;
    let distance_to_intercept = ((dx_pursuit*dx_pursuit + dy_pursuit*dy_pursuit) as f64).sqrt();

    // Calculate the time to intercept
    let time_to_intercept = (distance_to_intercept / *pursuer_speed as f64).round() as i32;

    // If the time to intercept is greater than the time to destination, the pursuer will not catch the target
    if time_to_intercept > time_to_destination {
        return None;
    }

    // Calculate the intercept angle
    let intercept_angle = (dy_pursuit as f64).atan2(dx_pursuit as f64) * 180.0 / PI;

    // If the intercept angle is less than the maximum intercept angle, return None
    if intercept_angle.abs() < min_intercept_angle {
        return None;
    }

    // Return the predicted target position
    Some(predicted_target_position)
}

/**
    TODO
    Calculate the optimal turning radius for each destination after the course is mapped
*/
fn calculate_optimal_turning_radius() -> (i32, i32) {
    (0, 0)
}


/**
    Calcuates the opponent's current velocity based on their x, y movement. Returns a tuple of
    their x velocity, y velocity, and speed
*/
fn determine_velocity(current: &Position, prev: &Position) -> Velocity {
    let a = f64::sqrt((current.x - prev.x) as f64).round().abs() as i32;
    let b = f64::sqrt((current.y - prev.y) as f64).round().abs() as i32;
    let speed = a + b;

    Velocity {
        x: a,
        y: b,
        speed: speed,
    }
}
