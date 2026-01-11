extern crate core;

// --- 1. Core Data Structure ---
// We define it with a unique name to avoid any macro hygiene conflicts,
// then alias it for easy use.
pub struct SolutionStruct {
    pub year: u32,
    pub day: u32,
    pub wrapper: fn(&str) -> (String, String),
}

pub use crate::SolutionStruct as Solution;

// --- 2. Year Module Declarations ---
// These tell Rust to look for src/year2024/mod.rs and src/year2025/mod.rs
pub mod year2024;
pub mod year2025;

// --- 3. Utility Module Declarations ---
// If you want to keep the library! macro for utils (which usually don't have mod.rs files),
// you can. Otherwise, standard 'pub mod utils;' also works.
macro_rules! library {
    ($name:ident $description:literal $($sub:ident),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $name {
            $(pub mod $sub;)*
        }
    }
}

library!(utils "Utility functions" grid, disjointset, read_lines, simplex, interval_tree, point);

// --- 4. The Registration Macro ---
// This is exported so it can be used inside your year/mod.rs files.
#[macro_export]
macro_rules! register_year {
    ($year:expr, $($day_mod:ident),*) => {
        // This declares each day file (day01.rs, etc.) as a module of the year
        $(pub mod $day_mod;)*

        pub fn get_solutions() -> Vec<$crate::Solution> {
            vec![
                $(
                    $crate::Solution {
                        year: $year,
                        day: stringify!($day_mod)
                            .strip_prefix("day")
                            .unwrap_or("0")
                            .parse()
                            .unwrap(),
                        wrapper: |data| {
                            // 'self' refers to the module where the macro is called (e.g., year2025)
                            match self::$day_mod::main(data) {
                                Ok((p1, p2)) => (p1.to_string(), p2.to_string()),
                                Err(e) => (format!("Error: {}", e), String::from("???")),
                            }
                        }
                    }
                ),*
            ]
        }
    }
}