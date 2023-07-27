# Matrix-framework Rust for Simula Summer 2023

## Prerequisites

* git
* rust (https://rustup.rs/)

## How to run

Clone the repository

run "cargo run" to run the code in debug mode.

run "cargo run -r" to run the code in release mode (significantly faster than debug mode).

## Structure of the repository

Code lies under the "src" folder.

Stored information about previously generated abelian groups is stored under the "data" folder. Only use the code to interact with these.

Created spreadsheets can be found under the "spreadsheets" folders. The "complete" spreadsheets contain all the information. The "summary" spreadsheets condense every quasigroup into it's automorphism fingerprints. The number after "Aut:" is the amount of automorphisms in this cell for every quasigroup in with this fingerprint. By definition should the "Aut:xx" value be identical for every permutation where it's non-zero for any given fingerprint. "AAut:" is the number of cells with this fingerprint where there is _at least one_ automorphism. So if a cell with that fingerprint has two different affine automorphisms, it will still only count as one.

The various pdfs that can be found is the latex compiled tables. The 5 x 5 table in "5by5.pdf" is only the first 1000 lines and was created for debug purposes.

## How the code is structured

All the structs are inside the "structs" module.

Any implementation that is not part of a new trait lies with its corresponding struct.

Any new traits is found in the "structs::traits" module. Any implementation corresponding to these traits are found with their respective trait definitions.

The "obsolete" module contains all the code that was used by the abelian group generating and manipulating code as well as old code used for the ASCII tables of latin squares.

Certain experiments that have been run can be seen under "obsolete/experiments". In order to run experiments, type "obsolete::experiments::{abelian, affine_automorphism, latin_square}::{name of experiment}" in the main function.