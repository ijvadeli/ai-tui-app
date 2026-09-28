# Implementation Plan

## Goal

Build a small Rust terminal app that prints a simple owl and has it present a randomly selected quote from *The Lord of the Rings* or *The Hobbit*. Show the character who says the quote as the attribution instead of the generic sign-off.

## Scope

- Keep the app as a straightforward command-line program with a small number of focused Rust functions.
- Use a simple, readable owl rendered as multiline ASCII art, inspired by the reference in [`src/app.md`](src/app.md).
- Keep quote text and its speaker together so the displayed attribution always matches the selected quote.
- Choose one quote for each run and print the owl, quote, and speaker in a clear layout.
- Keep quote text to material that is appropriately authorized for use; do not add unlicensed book excerpts.
- Do not add interactive controls, configuration, or a more elaborate terminal UI.

## Design

- Represent each quote as a text-and-speaker pair and store the available pairs in one collection.
- Keep quote selection, owl rendering, and output formatting in separate, small functions.
- Select a valid collection entry at random on each run. Prefer the simplest reliable approach compatible with the project; avoid adding a dependency unless it is needed.
- Have the entry point coordinate selection and display, and report any selection error through the program's normal error path.

## Implementation steps

1. Confirm the existing crate structure and retain its dependency-light setup unless a reliable random-selection approach requires a dependency.
2. Define the quote-and-speaker data and populate it only with appropriately sourced content.
3. Implement random selection and the simple multiline owl as focused functions.
4. Implement output formatting so the selected quote is followed by its character attribution.
5. Add focused tests for quote data validity, selection returning an entry in the collection, and owl/output formatting.
6. Run formatting, tests, and the app; adjust the layout if the terminal output is unclear.

## Acceptance criteria

- Running the app prints a recognizable multi-line owl and one quote.
- The quote has a non-empty speaker attribution, and the speaker is associated with that exact quote.
- A quote is selected from the available collection on each run without an out-of-range selection.
- The implementation remains small and uses no unnecessary dependencies or unrelated UI features.
- Tests cover the quote collection, selection, and rendered output.
- All included quote text is appropriately authorized for use.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run`

