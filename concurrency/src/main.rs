// use std::env;

// #[derive(Debug, Clone, Copy)]
// enum Operation {
//     Forward(isize),
//     TurnLeft,
//     TurnRight,
//     Home,
//     Noop(u8),
// }

// #[derive(Debug)]
// enum Orientation {
//     East,
//     North,
//     South,
//     West,
// }

// #[derive(Debug)]
// struct Artist {
//     x: isize,
//     y: isize,
//     heading: Orientation,
// }

// const WIDTH: isize = 400;
// const HEIGHT: isize = WIDTH;
// const HOME_Y: isize = HEIGHT / 2;
// const HOME_X: isize = WIDTH / 2;
// const STROKE_WIDTH: usize = 5;

// fn parse(input: &str) -> Vec<Operation> {
//     let mut steps = Vec::<Operation>::new();
//     for bytes in input.bytes() {
//         let step = match bytes {
//             b'0' => Operation::Home,
//             b'1'..=b'9' => {
//                 let distance = (bytes - 0x30) as isize;
//                 Operation::Forward(distance * (HEIGHT / 10))
//             }
//             b'a' | b'b' | b'c' => Operation::TurnLeft,
//             b'd' | b'e' | b'f' => Operation::TurnRight,
//             _ => Operation::Noop(bytes),
//         };
//         steps.push(step);
//     }
//     steps
// }
// impl  {
    
// }



// fn main() {
//     let args = env::args().collect::<Vec<String>>();
//     let input = args.get(1).unwrap();
//     let default = format!("{}.svg", input);
//     let save_to = args.get(2).unwrap_or(&default);
//     let operation = parse(input);
// }


fn main() {
    s
}
