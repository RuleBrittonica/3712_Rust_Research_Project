# Change Log

All notable changes to this project will be documented in this file.

## [0.1.0] 2024-10-03

### Added
- Initial release
- Mostly functional extraction, a few rough edges where error handling is not
  ideal

## [0.1.1] 2024-10-11
- Modified to return the String of the extracted text instead creating a new
  file with the extracted text (and the returning the `PathBuf` to that file)
- Testing still produces extra files for the review, but the actual extraction
  is now done in memory
- This is to make it compatible with rem-cli.

## [0.1.2] 2024-10-11
- Added the return of the parent function from the given range. 