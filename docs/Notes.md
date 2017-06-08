## Notes on the design and implemenation of the game

### Disclaimer

Note these are not in any organised fashion. On the contrary, these are more like scribblings as
I undertake the design and implementation of the game. Since this project is of very small size, a
comprehensive design document seemed like overkill.


### Overall Design

The project will be divided into the following modules:

  * io - this module will handle all the I/O related functionality in a more or less
         independent and abstracted fashion.

  * error_handling - this module will take care of all the error handling needs of the other
                     modules.
  * gane_logic - this is the core of game containing all the business logic of the game itself
                 including the visual elements.
  * validation - this is a submodule of `game_logic` that will handle all the validation needs of
                 the overall game, and will therefore have a lot of interaction with the
                 `error_handling` module as well as the `io` module.


### Storing the game elements

Given the rules of the `nogo` game, the following data structures will be used:

  * ```
    struct NogoPlayer {
           id: u8, // 0, or 1
           strings: Vec<Point>,
           human: bool,
    }
    ```

    This represents a player in the game. There are only a maximum of two players in the game
    at any point of time, and and `id` of 0 will map to "Player 1", and 1 to "Player 2". The
    `human` flag will indicate whether the player is a human or an automaton.
    

    ```
  *   struct Point {
           x: u8,
           y: u8,
    }
    ```

    This represents a coordinate in the Cartesian plane of the game.


### Main logic

In the case of a human player, the player will supply the moves, and validation will ensure that
only a valid move is accepted.

In the case of the computer, the algorithm for generating the next move is specified in the
assignment document.




    
 
