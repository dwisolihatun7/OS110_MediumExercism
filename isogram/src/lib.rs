use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen: HashSet<char> = HashSet::new();
    for c in candidate.to_lowercase().chars(){
        if !c.is_alphanumeric() {continue}
        if seen.contains(&c) {return false}
        seen.insert(c);
    }
    //unimplemented!("Is {} an isogram?", candidate);
    true
}
