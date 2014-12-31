

## Architecture

main - connects two independent domains
  - game - contains the game logic, i.e. the behaviour
  - mob - state for a mob, and atomic functions for operating on it
  - io as in input and output, for now this includes rendering
