# Contributing to Reeverb

Thanks for your interest in contributing. Here's how to get started.

## Setup

1. Fork the repo and clone your fork
2. Install Rust 1.85+ via [rustup](https://rustup.rs)
3. Install PostgreSQL 16+ (or use Docker: `docker compose up db -d`)
4. Copy `.env.example` to `.env` and configure your database
5. Run `cargo run` to start the server
6. Run `cargo test` to run the test suite

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
