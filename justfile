# justfile

[private]
default:
    @just --list

alias b := build
alias r := run
alias c := clippy

# targets := ["esp32", "esp32c3"]

[group('cargo')]
build package board="esp32c3":
    cargo +esp build --target {{ if board == "esp32" { "xtensa-esp32-none-elf" } else { "riscv32imc-unknown-none-elf" } }} --features {{board}} -p {{ package }} -Z build-std=core

[group('cargo')]
run package board="esp32c3":
    cargo +esp run --target {{ if board == "esp32" { "xtensa-esp32-none-elf" } else { "riscv32imc-unknown-none-elf" } }} --features {{ board }} -p {{ package }} -Z build-std=core

[group('cargo')]
clippy package board="esp32c3":
    cargo +esp clippy --target {{ if board == "esp32" { "xtensa-esp32-none-elf" } else { "riscv32imc-unknown-none-elf" } }} --features {{ board }} -p {{ package }} -Z build-std=core

[group('release')]
release package board="esp32c3":
    cargo +esp run --target {{ if board == "esp32" { "xtensa-esp32-none-elf" } else { "riscv32imc-unknown-none-elf" } }} --features {{ board }} --release -p {{ package }} -Z build-std=core

watch package board="esp32c3":
    cargo +esp watch -x 'clippy --target {{ if board == "esp32" { "xtensa-esp32-none-elf" } else { "riscv32imc-unknown-none-elf" } }} --features {{ board }}' -p {{ package }} -Z build-std=core

test package board="esp32c3":
    cargo nextest run --features {{ board }} -p {{ package }}

[group('ci')]
prepare package: fmt (_prepare_all package)

[group('ci')]
fix board="esp32c3":
    cargo +esp clippy --fix --target {{ if board == "esp32" { "xtensa-esp32-none-elf" } else { "riscv32imc-unknown-none-elf" } }} --features {{ board }} --allow-dirty -Z build-std=core

[group('ci')]
fmt: taplo
    cargo +nightly fmt -- --config-path ./rustfmt.nightly.toml

taplo: 
    @taplo fmt

_ci_fmt:
    cargo +nightly fmt --all -- --config-path ./rustfmt.nightly.toml --check --color always

_ci_build package board: (build package board)

_ci_clippy package board:
    cargo +esp clippy --target {{ if board == "esp32" { "xtensa-esp32-none-elf" } else { "riscv32imc-unknown-none-elf" } }} --features {{ board }} -p {{ package }} -Z build-std=core -- -D warnings 

_ci_test package board: (test package board)

_prepare package board: (_ci_clippy package board) (_ci_build package board)

_prepare_all package: (_prepare package "esp32c3") (_prepare package "esp32")