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

astro:
    bun run build

test package board="esp32c3":
    cargo nextest run --features {{ board }} -p {{ package }}

[group('ci')]
prepare: fmt _prepare_all

[group('ci')]
fix board="esp32c3":
    cargo +esp clippy --fix --target {{ if board == "esp32" { "xtensa-esp32-none-elf" } else { "riscv32imc-unknown-none-elf" } }} --features {{ board }} --allow-dirty -Z build-std=core

[group('ci')]
fmt: taplo
    cargo +nightly fmt -- --config-path ./rustfmt.nightly.toml

taplo: 
    @taplo fmt

[group('ci')]
_ci_fmt:
    cargo +nightly fmt --all -- --config-path ./rustfmt.nightly.toml --check --color always

_ci_build board: astro (build board)

[group('ci')]
_ci_clippy board: astro
    cargo +esp clippy --target {{ if board == "esp32" { "xtensa-esp32-none-elf" } else { "riscv32imc-unknown-none-elf" } }} --features {{ board }} --workspace -- -D warnings 

_prepare board: (_ci_clippy board) (_ci_build board)

_prepare_all: astro (_prepare "esp32c3") (_prepare "esp32")
