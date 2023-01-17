
# Tic Tac Toe - MENACE Edition

![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/AliSajid/ttt_menace?include_prereleases)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/AliSajid/ttt_menace)
[![Continuous integration](https://github.com/AliSajid/ttt_menace/actions/workflows/ci.yaml/badge.svg)](https://github.com/AliSajid/ttt_menace/actions/workflows/ci.yaml)
[![Contribute with Gitpod](https://img.shields.io/badge/Contribute%20with-Gitpod-908a85?logo=gitpod)](https://gitpod.io/#https://github.com/AliSajid/ttt_menace)


![GitHub issues](https://img.shields.io/github/issues/AliSajid/ttt_menace)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/AliSajid/ttt_menace)
![GitHub (Pre-)Release Date](https://img.shields.io/github/release-date-pre/AliSajid/ttt_menace)
![GitHub commits since latest release (by SemVer including pre-releases)](https://img.shields.io/github/commits-since/AliSajid/ttt_menace/latest?include_prereleases&sort=semver)
![GitHub contributors](https://img.shields.io/github/contributors/AliSajid/ttt_menace)



[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/AliSajid/ttt_menace)

This is a toy project to create a Tic Tac Toe game that can be played by any two players. The ultimate intention here is to create the game in such
a way that it can be played by a human and a computer. The computer will be using the MENACE algorithm to learn how to play the game.

## Builds

| Platform | Rust Version |Status |
| -------- | ------ | ------ |
| Linux    | stable <br/> beta <br/> nightly <br/> MSRV (1.58.1) | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-stable.json) <br/> ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-beta.json) <br/> ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-nightly.json) <br/> ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-msrv.json) |
| Windows  | stable <br/> beta <br/> nightly <br/> MSRV (1.58.1) | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-stable.json) <br/> ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-beta.json) <br/> ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-nightly.json) <br/> ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-msrv.json) |
| macOS    | stable <br/> beta <br/> nightly <br/> MSRV (1.58.1) | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-stable.json) <br/> ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-beta.json) <br/> ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-nightly.json) <br/> ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-msrv.json) |


## MENACE

Machine Educable Noughts and Crosses Engine (MENACE) is one of the first implementations of a mchine learning system. It was developed by
Donald Michie in 1961. The original system was developed using a stack of matchboxes over a period of time and was called Matchbox Educable
Noughts and Crosses Engine (MENACE).

This was one of the first systems to use reinforcement learning to learn how to play a game and the first to prove that
a machine could learn how to play a game without being explicitly programmed to do so.

The classical MENACE system consisted of 304 matchboxes. Each matchbox represented a possible state of the game. Each matchbox had a
number of beads inside, with the number and color of beads representing the next move to be made. The system would play a game of Tic Tac Toe
and keep track of the moves made throughout the game. Afterwards, depending on the result of the game (win, lose, draw), the system would either be "rewarded" or "punished" by adding or removing beads from the matchboxes. The system would then play another game and repeat the process.

More information on MENACE can be found [here](https://en.wikipedia.org/wiki/MENACE).

## Project Structure

The project is structured as follows:

- A binary crate called `tttm` which contains the main executable called `tttm`.
- A librrary crate called `ttt_menace_lib` which contains the core logic of the game, alongwith the replaceable player logic.

## MENACE Implementation

Since MENACE was originally designed to not require a computer, we have to make certain adjustments in adapting the Matchbox-based system to a computer-based system. We will follow the following principles in our implementation:

- The matchboxes will be represented by a 2D array of `u8` values. Each value will represent the number of beads in the matchbox.
- We will use a HashMap to store the state of the game. The key will be a string representation of the board state and the value will be the index of the matchbox in the 2D array.

Additionally, the original MENACE implementation used a manually curated list of possible game states that treated the rotationally symmetrical board states as equivalent. Since we are not constrained by the number of virtual matchboxes, we will be implementing the MENACE system in two flavors:

1. MENACE-C: This will be the classic MENACE system that treats the rotationally symmetrical board states as equivalent.
2. MENACE-S: This will be the MENACE system that treats the rotationally symmetrical board states as distinct.

## Roadmap

The project is currently in the early stages of development. The following is a list of features that will be implemented in the future:

- [ ] Implement the core logic of the game.
- [ ] Implement the MENACE-C system.
- [ ] Implement the MENACE-S system.
- [ ] Implement the CLI.
- [ ] Add a TUI using [tui-rs](https://github.com/fdehau/tui-rs)

## Contributing

Contributions to the project are welcome. Please see the [Contributing Guidelines](CONTRIBUTING.md) for more information.
