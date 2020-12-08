#[derive(Debug, PartialEq, Eq)]
struct BoardingPass {
    row: u64,
    column: u64,
    seat_id: u64,
}

fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input = String::from_utf8(input).expect("Input file is not valid UTF-8.");

    let boarding_passes: Vec<BoardingPass> = input.lines()
        .map(|l| to_boarding_pass(l.to_string()))
        .collect();

    let highest_seat_id = boarding_passes.iter().max_by(|b1, b2| b1.seat_id.cmp(&b2.seat_id)).expect("No boarding passes in input").seat_id;
    println!("The highest seat ID among the boarding passes is: {}", highest_seat_id);

    let my_seat_id: u64 = (0..=0b11_1111_1111).find(|i| {
            !boarding_passes.iter().any(|b| b.seat_id == *i)
            && boarding_passes.iter().any(|b| b.seat_id == *i + 1)
            && boarding_passes.iter().any(|b| b.seat_id == *i - 1)
        })
        .unwrap();
    println!("My seat ID is: {}", my_seat_id);
}

fn to_boarding_pass(input: String) -> BoardingPass {
    let bits: u64 = input.chars()
        .fold(0, |bits, c| {
            (bits << 1) | match c {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => panic!("Not a valid boarding pass."),
            }});
    let row: u64 = bits >> 3;
    let column: u64 = bits & 0b111;

    BoardingPass {
        row: row,
        column: column,
        seat_id: row * 8 + column,
    }
}
