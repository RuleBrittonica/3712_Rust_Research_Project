# ENGN3712 (Rust Research Project with Alex Potanin and Sasha Pak)
This repository contains the code and artefacts for my Rust research project
undertaken during semester 2, 2024, and semesters 1 and 2, 2025, at The
Australian National University.
During this project I was supervised by Dr Alex Potanin and Mr Sasha Pak (PhD
candidate).

## Structure

This repository contains the following directories:

 - evaluation: contains all code used to evaluate the REM toolchain. This
   includes 80x repo clones and 20x test crates used for benchmarking and
   evaluation. I have done my best to strip out any large files from this
   folder, but it does make for a weighty repository.
 - artefacts: Contains a concrete build artefact using docker that allows you to
   build and run the REM toolchain, along with any associated dependencies. (TODO)
 - experiments: Contains experimental code used to test concepts and ideas.
 - rem: Contains the New Rust-Extraction-Maestro (REM) toolchain, and all
   surrounding infrastructure. This is the main
   codebase for the project.
 - reports: Contains the reports for the project (mid-semester, final, seminar
   and SPLASH2025 presentation). Reports were written in Overleaf and thus may
   not be up to date here.


### Reports

 - enivironment test: Ensure that the latex environment builds correctly
 - mid_semester: (Due Friday week 12, semsester 2, 2024) A report on the progress of the project
 - final: (Due 3-4 weeks after Friday week 12, semester 2, 2025) The final
   report for the project
 - seminar: (Due Wednseday week 12, semester 2, 2025) A presentation report for
   the research seminar
 - splash_poster: A poster and extended abstract on the verification component of
   the REM toolchain, submitted to SPLASH2025, and presented by Mr Sasha Pak.
