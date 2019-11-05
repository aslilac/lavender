/// The 32-bit ARM instructions and decoder
pub mod arm;
/// The 16-bit Thumb instructions and decoder
pub mod thumb;
/// Some useful things for processing instructions that I felt would be better
/// suited to another file, rather than crowding an already large file.
pub mod utils;

#[cfg(test)]
mod arm_tests;
#[cfg(test)]
mod thumb_tests;
