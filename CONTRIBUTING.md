# Contributing to AegisForge

First off, thank you for considering contributing to AegisForge! It's people like you that make AegisForge such a great tool for the Discord community.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please be respectful and professional in all interactions.

## How Can I Contribute?

### Reporting Bugs

- **Check for existing issues**: Before opening a new issue, please search the tracker to see if it has already been reported.
- **Use a clear title**: Provide a concise summary of the problem.
- **Describe the steps to reproduce**: Include the command used, the expected outcome, and the actual result.
- **Provide context**: Mention your environment (OS, Rust version) and any relevant logs.

### Suggesting Enhancements

- **Open an issue**: Explain the feature you'd like to see and why it would be useful.
- **Be descriptive**: Provide examples of how the feature would work.

### Pull Requests

1. **Fork the repository**.
2. **Create a new branch** for your feature or bugfix (`git checkout -b feature/amazing-feature`).
3. **Write your code**: Ensure it follows the project's style and is well-documented.
4. **Run tests**: Make sure everything works as expected.
5. **Commit your changes** (`git commit -m 'Add some amazing feature'`).
6. **Push to the branch** (`git push origin feature/amazing-feature`).
7. **Open a Pull Request**.

## Style Guide

- **Language**: Use idiomatic Rust.
- **Formatting**: Run `cargo fmt` before committing.
- **Linting**: Ensure `cargo clippy` passes without warnings.
- **Documentation**: Use `///` for doc comments on public items.

## Development Environment

- **Rust**: Latest stable version.
- **Database**: We use Neon PostgreSQL for production and local development. Ensure you have `sqlx-cli` installed for migrations.

## Questions?

If you have any questions, feel free to join our [Discord Server](https://discord.gg/your-invite-link) or open an issue for discussion.

Happy coding! 🦀
