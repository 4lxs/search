import 'just-flake.just'

default:
    @just --list

# Run 'cargo run' on the project
run *ARGS:
    cargo run {{ARGS}}

bacon:
    bacon
