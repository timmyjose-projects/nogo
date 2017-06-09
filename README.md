A simple clone of the Atari Go game.

This game is based on The University of Queensland's School of Information Technology and Electrical Engineering
course CSSE2310/CSSE7231 course assignment.


## How to play the game?

There are two players (either human or automated) represented by board pieces 0's and X's respectively. The rules are
quite simply as follows:

  * A group of similar pieces is considered to be a string when they are all horizontally or vertically connected.
  * An empty cell horizontal or vertical to a string is considered a "liberty", or space into which the string can grow.
  * When a string has no liberties left, the string is considered to be captured.
  * The player whose string (any string) gets captured is considered to have lost the game.
  * In the case of a move which could result in either player's string(s) getting captured, the player who has the
    current move gets the option of choosing the winning move.

At each step of the game, the player (human or computer) will enter the `row` and `column` of the next move (that is, where to
place the next piece in). Only a valid move will be accepted. Also note that the user must enter the `row` and `column` values
using normal 0-based indexing counting down from the top-left of the board. For instance, move `1 2` refers to the second row, third column (the top-left position of the board has coordinates `0 0`)


## Usage

If running using Cargo,

```
$ cargo run nogo p1type p2type [height width | filename]
```

where `p1type` and `p2type` can be `c` (computer) or `h` (human).

`height` and `width` representing the size of the board represented as cells must be integrs between 4 and 1000 inclusive.

In case `height` and `width` are not specified, the assumption is that a `filename` is provided, which contains a saved game.


If running using the executable directly,

```
$ nogo p1type p2type [height width | filename]
```

All terms have the same exact meaning as explained previously.


## Saving games

As mentioned in the previous section, a previously played (incomplete) game can be loaded instead of starting a new game.

To save the game during a session, instead of entering a row and column (assuming a human player, of course), enter `w` followed
by the full path of the file to save the game state to. Note that there must be no space between `w` and the file path.


## Sample gameplay

To assist in getting started out with the game, here is a small gameplay between a human (player 1) and the computer (player 2).





