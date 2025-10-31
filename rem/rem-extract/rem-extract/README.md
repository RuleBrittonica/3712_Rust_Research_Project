# rem-extract

A fully self contained initial function extraction CLI tool for Rust Code.
Uses Rust-Analyzer internally to extract the function signature and generate a
new file with the extracted function. Significantly faster than spooling up an
entire instance of Rust-Analyzer to just extract a function. Used as a
preprocessor for the REM toolchain to extract functions from a file before
fixing the lifetimes etc. 
