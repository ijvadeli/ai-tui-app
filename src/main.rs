use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

const SAYINGS: &[&str] = &[
    "Even the longest road begins beneath your feet.",
    "A warm light can make a small home feel like a kingdom.",
    "Courage is taking the next step while the path is still dark.",
    "The best maps leave room for an unexpected turn.",
    "A shared meal can turn strangers into companions.",
    "Keep a little wonder for the road ahead.",
    "Small hands can open doors that seem far too heavy.",
    "The quietest traveler may carry the brightest hope.",
];

fn random_saying() -> Result<&'static str, std::time::SystemTimeError> {
    let elapsed = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let mut hasher = RandomState::new().build_hasher();
    elapsed.as_nanos().hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    let index = (hasher.finish() as usize) % SAYINGS.len();
    Ok(SAYINGS[index])
}

fn main() -> Result<(), std::time::SystemTimeError> {
    let saying = random_saying()?;

    println!("          ,_,");
    println!("         (O,O)");
    println!("         (   )");
    println!("        /|   |\\");
    println!("       /_|___|_\\");
    println!("         /   \\");
    println!("        /_____\\");
    println!();
    println!("  \"{saying}\"");
    println!("  -- A saying for the road");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{SAYINGS, random_saying};

    #[test]
    fn selected_saying_is_from_the_collection() {
        let saying = random_saying().expect("the system clock should be available");
        assert!(SAYINGS.contains(&saying));
    }

    #[test]
    fn sayings_are_non_empty() {
        assert!(SAYINGS.iter().all(|saying| !saying.trim().is_empty()));
    }
}
