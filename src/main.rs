use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

struct Quote {
    text: &'static str,
    speaker: &'static str,
}

const QUOTES: &[Quote] = &[
    Quote {
        text: "Not all those who wander are lost.",
        speaker: "Bilbo Baggins",
    },
    Quote {
        text: "You shall not pass!",
        speaker: "Gandalf",
    },
    Quote {
        text: "My precious.",
        speaker: "Gollum",
    },
    Quote {
        text: "There and back again.",
        speaker: "Bilbo Baggins",
    },
    Quote {
        text: "The road goes ever on and on.",
        speaker: "Bilbo Baggins",
    },
    Quote {
        text: "Even the smallest person can change the course of the future.",
        speaker: "Galadriel",
    },
    Quote {
        text: "A wizard is never late.",
        speaker: "Gandalf",
    },
    Quote {
        text: "I am no man!",
        speaker: "Éowyn",
    },
];

fn random_quote() -> Result<&'static Quote, std::time::SystemTimeError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let mut hasher = RandomState::new().build_hasher();
    now.as_nanos().hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    let index = hasher.finish() as usize % QUOTES.len();
    Ok(&QUOTES[index])
}

fn owl() -> &'static str {
    r#"      ,_,
     (O,O)
     (   )
    /|   |\
   /_|___|_\
     /   \
    /_____\"#
}

fn display_quote(quote: &str, speaker: &str) {
    println!("{}", owl());
    println!();
    println!("  \"{quote}\"");
    println!("  -- {speaker}");
}

fn main() -> Result<(), std::time::SystemTimeError> {
    let quote = random_quote()?;
    display_quote(quote.text, quote.speaker);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{QUOTES, owl, random_quote};

    #[test]
    fn selected_quote_is_from_the_collection() {
        let quote = random_quote().expect("the system clock should be available");
        assert!(QUOTES
            .iter()
            .any(|candidate| candidate.text == quote.text && candidate.speaker == quote.speaker));
    }

    #[test]
    fn quotes_are_non_empty() {
        assert!(
            QUOTES
                .iter()
                .all(|quote| { !quote.text.trim().is_empty() && !quote.speaker.trim().is_empty() })
        );
    }

    #[test]
    fn owl_has_multiple_lines() {
        assert!(owl().lines().count() > 1);
    }
}
