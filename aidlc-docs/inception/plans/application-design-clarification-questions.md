# Application Design Clarification Questions

## Contradiction 1: TUI Framework vs Tech Stack
You selected Charm (charm.land) as the TUI framework, but Charm's tools (Bubble Tea, Lip Gloss, Huh) are Go-based libraries. The project tech stack is TypeScript/Node.js. These are incompatible.

### Clarification Question 1
How should we resolve the TUI framework vs TypeScript tech stack conflict?

A) Switch to a TypeScript-compatible TUI library — Ink (React for CLI) with ink-select, ink-text-input, etc.
B) Switch to a TypeScript-compatible TUI library — Blessed/neo-blessed (terminal UI toolkit)
C) Switch to a TypeScript-compatible TUI library — Prompts (enquirer or prompts) for interactive CLI
D) Change the project language to Go and use Charm's Bubble Tea
E) Other (please describe after [Answer]: tag below)

[Answer]: E — Switch to Rust + Ratatui TUI framework (user confirmed language change)A
