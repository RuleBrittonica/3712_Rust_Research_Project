pub const ABOUT: &str = r#" A preprocessing tool for the rem-toolchain.
Takes a source code file and:
    - Extracts a method from the specified line range (just copies and pastes the code)
    - Places the method below the caller function
    - Performs type inference on the extracted method to give it a function signature
    - Formats the code witth rustfmt

Utilises:
    - rem-utils: git= https://github.com/RuleBrittonica/rem-utils

Used in:
    - rem-cli: git= https://github.com/RuleBrittonica/rem-cli
"#;
