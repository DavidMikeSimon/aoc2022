use std::{error, fs, collections::{VecDeque, HashSet}};

const PACKET_LEN: usize = 14;

fn main() -> Result<(), Box<dyn error::Error>> {
    let s = fs::read_to_string("./data/day6.txt")?;
    let s = s.trim();

    let mut buf = VecDeque::<char>::new();
    for (idx, c) in s.chars().enumerate() {
        buf.push_back(c);
        if buf.len() > PACKET_LEN {
            buf.pop_front();
            let set: HashSet<char> = buf.iter().copied().collect();
            if set.len() == PACKET_LEN {
                dbg!(idx+1);
                break;
            }
        }
    }

    Ok(())
}
