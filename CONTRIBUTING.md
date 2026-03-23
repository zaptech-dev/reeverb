# Contributing to Reeverb

Thanks for your interest in contributing. Here's how to get started.

## Setup

1. Fork the repo and clone your fork
2. In the root directory run `git remote add upstream https://github.com/zaptech-dev/reeverb`
3. Install Rust 1.85+ via [rustup](https://rustup.rs)
4. Install PostgreSQL 16+ (or use Docker: `docker compose up db -d`)
5. Copy `.env.example` to `.env` and configure your database
6. Run `cargo install trunk`
7. Run `rustup target add wasm32-unknown-unknown`
8. `cd` into `reeverb/crates/dashboard` and run `trunk build`
9. Go back to the root directory and run `cargo run` to start the server
10. Run `cargo test` to run the test suite

## Pull Requests

- Keep PRs focused on a single change
- Write tests for new functionality
- Run `cargo clippy` and `cargo fmt` before submitting
- Write clear commit messages (single line, imperative mood)

## Code Style

- Follow existing patterns in the codebase
- Feature-first module structure (group by domain, not by layer)
- Use Rapina conventions: `#[get]`, `#[post]`, `Validated<T>`, typed errors
- Handlers: `list_users`, `create_user`, `get_user`
- DTOs: `CreateUserRequest`, `UserResponse`
- Routes: plural, versioned (`/v1/users/:id`)

## Issues

- Use GitHub Issues for bugs and feature requests
- Include reproduction steps for bugs
- Check existing issues before opening a new one

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
