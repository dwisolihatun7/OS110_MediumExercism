/// Compute the Scrabble score for a word
//#![feature(iter_arith)]

fn Letter_Value(c:char) -> u32{
    match c{
        'a'| 'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
        'd'|'g' => 2,
        'b'| 'c'|'m'|'p' => 3,
        'f'| 'h'|'v'|'w'|'y'=> 4,
        'k' =>5,
        'j'|'x' =>8,
        'q'|'z' => 10,
        _ =>0,
    }
}

pub fn score(word: &str) -> u32 {
    word.chars().map(|c| Letter_Value(c.to_lowercase().next().unwrap())).sum()
    //unimplemented!("Score {} in Scrabble.", word);
}
