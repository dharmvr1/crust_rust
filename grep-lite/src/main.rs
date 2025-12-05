use std::{
    fs::File,
    io::{self, BufRead, BufReader, Stdin},
};

use clap::{App, Arg};
use regex::Regex;
fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("pattern match")
        .arg(
            Arg::with_name("Pattern")
                .help("The Pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("file to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("Pattern").unwrap();

    let re = regex::Regex::new(pattern).unwrap();
    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_line(reader, re);
    } else {
        let file = File::open(input).unwrap();
        let buf = BufReader::new(file);
        process_line(buf, re);
    }
}

fn process_line<T: BufRead + Sized>(redear: T, re: Regex) {
    for line in redear.lines() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

//   let quote = "Every face, every shop, bedroom window, public-house, and
// dark square is a picture feverishly turned--in search of what?
// It is the same with books. What do we seek through millions of pages?";
//     for line in quote.lines() {
//         let line_contained = re.find(line);
//         match line_contained {
//             Some(_) => println!("{:?}", line),
//             None => (),
//         }
//     }

//     let ctx_lines = 2;
//     let needles = "oo";
//     let haystack = "/
//    Every face, every shop,
//    bedroom window, public-house, and
//    dark square is a picture
//    feverishly turned--in search of what?
//    It is the same with books.
//    What do we seek
//    through millions of pages?";

//     let mut tags: Vec<usize> = vec![];
//     let mut ctx: Vec<_> = vec![];

//     for (i, line) in haystack.lines().enumerate() {
//         if line.contains(needles) {
//             tags.push(i);

//             let v: Vec<_> = Vec::with_capacity(2 * ctx_lines + 1);

//             ctx.push(v);
//         }
//     }
//     if tags.is_empty() {
//         return;
//     }

// for (i, line) in haystack.lines().enumerate() {
//     for (j, tag) in tags.iter().enumerate() {
//         let lower_bound = tag.saturating_sub(ctx_lines);

//         let upper_bound = tag + ctx_lines;

//         if (i >= lower_bound) && (i <= upper_bound) {
//             let line_as_string = String::from(line);
//             let local_ctx = (i, line_as_string);
//             ctx[j].push(local_ctx);
//         }
//     }
// }

// for local in ctx {
//     for (j, line) in local.iter() {
//         println!("{}:{}", j, line.trim());
//     }
// }
