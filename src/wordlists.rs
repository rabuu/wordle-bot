#[cfg(not(feature = "6mal5"))]
pub const POSSIBLE_SOLUTIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/wordlists/nytimes.com/possible_solutions"
));

#[cfg(not(feature = "6mal5"))]
pub const EXTRA_GUESSING_OPTIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/wordlists/nytimes.com/extra_guessing_options"
));

#[cfg(feature = "6mal5")]
pub const POSSIBLE_SOLUTIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/wordlists/6mal5.com/possible_solutions"
));

#[cfg(feature = "6mal5")]
pub const EXTRA_GUESSING_OPTIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/wordlists/6mal5.com/extra_guessing_options"
));
