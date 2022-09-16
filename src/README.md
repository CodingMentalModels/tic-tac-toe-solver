# Rusty Tic-Tac-Toe

### Implementing the game primitives

Goal: Implement all of the primitives of tic tac toe in order to simulate a game and determine who's won or lost.

Primitives:
* Players (X, O)
* Board
    * Under the hood, this is two bitboards, one for X and one for O.  Each uses a 9 digit binary number to represent the squares that are populated and easily check, e.g. for victories.  
    * Board can determine outcomes
    * Board can be set by passing in a string
    * Board can be updated directly by calling `set(i, j)`
* Move
    * Simple two column vector
    * Can be instantiated from a string
    * Can be passed into a board to update the board.
* Columns, Rows, Diagonals -- win condition

### Building the game tree

Goal: Create a representation of the game tree, in order to model the space of all possible outcomes.

### Solving the game

Goal: Construct the entire game tree and determine the optimal move at each decision point.

### Implement a CLI for interacting with the solver

Goal: In the terminal, we should be able to call the solver on a certain position and get the results.