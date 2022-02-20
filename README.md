# wordle-bot
This is a bot for the game *Wordle* written in Rust.

*Wordle* is a web-based word guessing game developed by Josh Wardle.
You have to guess a random word based on the feedback you get from your previous guesses.

You can find the original version of the game [here](https://www.nytimes.com/games/wordle/index.html).

And a German version is available [here](https://6mal5.com/).

## Building & runnning
Currently, the word lists are included at *compile* time. This leads to a faster entropy calculation.
Therefore, however, the word lists must also be specified at compile time.
This done using *features*. These are specified in the [manifest](Cargo.toml) and set [here](src/wordlists.rs).

To compile with the default lists from [nytimes.com](https://www.nytimes.com/games/wordle/index.html):
```console
$ cargo build
```

To compile with the german lists from [6mal5.com](https://6mal5.com/):
```console
$ cargo build --features=6mal5
```
