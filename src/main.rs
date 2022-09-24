// Inspired by https://www.youtube.com/watch?v=gX6EFBICIcY

const DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ ","  ╻  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┏━┛ ", " ┣━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╹ "],
    ["┗━┛ ","  ╹  "," ┗━━ ", " ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
];

const TIMES_UP_BANNER : &str = "┌───────────────────────────────┐\n\
                                │===============================│\n\
                                │                               │\n\
                                │         Time is up!           │\n\
                                │                               │\n\
                                │===============================│\n\
                                └───────────────────────────────┘";

// https://www.w3.org/TR/xml-entity-names/025.html
fn draw_time_with_box_drawings(min:u32, sec:u32)-> () {
    print!("\x1b[?25l"); // hide cursor
    let time: String = min.to_string() + ":" + &sec.to_string();
    for row in &DIGITS {
        for c in time.chars() {
            let col = match c {
                '0'..='9' => c as usize - '0' as usize,
                ':' => 10,
                _ => 10,
            };
            print!("{} ", row[col]);
        }
        println!();
    }
    print!("\x1b[7A"); // move the cursor to the top, so msg is refreshed 
}

fn end_alarm() -> () {
    println!("{TIMES_UP_BANNER}");
    loop{
        print!("\x07");
    }
}

fn read_user_time() -> (u32, u32){
    let mut input_min = String::new();
    let mut input_sec = String::new();

    println!("Enter time to set");
    println!("min:");
    std::io::stdin()
        .read_line(&mut input_min)
        .expect("Failed to read line");
    println!("sec:");
    std::io::stdin()
        .read_line(&mut input_sec)
        .expect("Failed to read line");

    return (input_min.trim().parse().expect("Please type a number!"),
            input_sec.trim().parse().expect("Please type a number!"));
}

fn countdown_time(time_amount: (u32, u32)) -> () {
    let (usr_min, usr_sec) = time_amount;
    
    for min in 0..usr_min{
        for sec in 0..60 {
            draw_time_with_box_drawings(min, sec);
            std::thread::sleep(std::time::Duration::from_millis(999));
        }
    }
    
    for sec in 0..usr_sec{
            draw_time_with_box_drawings(usr_min, sec);
            std::thread::sleep(std::time::Duration::from_millis(999));
    }
}

fn main() {

    countdown_time(read_user_time());
    end_alarm()

}