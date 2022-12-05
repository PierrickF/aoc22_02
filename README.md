# Advent of Code 2022 - Day 2

## Task 1

The rules of rock-paper-scissors are as follows:
* Rock beats scissors, and is worth 1 point regardless of outcome.
* Paper beats rock, and is worth 2 points regardless of outcome.
* Scissors beats paper, and is  worth 3 points regardless of outcome.
* A round lost grants 0 point.
* A draw grants 3 points.
* A round won grants 6 points.

Parse the input file which represents rounds of rock-paper-scissors.
* The left column represents your opponent's moves.
* The right column represents your moves.
* Rock is `A` and `X`.
* Paper is `B` and `Y`.
* Scissors is `C` and `Z`.

Calculate your final score.

## Example

```
A Y
B X
C Z
```

Line 1 grants 2 points for playing paper and 6 points for winning against rock.\
`2 + 6 = 8`\
Line 2 grants 1 point for playing rock and 0 points for winning against rock.\
`1 + 0 = 1`\
Line 3 grants 3 points for playing scissors and 3 points for ending in a draw.\
`3 + 3 = 6`

Your final score is `8 + 1 + 6 = 15`.

## Task 2

The following rules have changed:
* The right column indicates the desired outcome of each round.
* `X` means you lose.
* `Y` means the round ends in a draw.
* `Z` means you win.

Deduce what you need to play to reach the desired outcome for each round.\
Calculate your final score.

## Example

```
A Y
B X
C Z
```

On line 1 you want to end in a draw `Y`, and earn `3` points.\
Your opponent played rock `A`.\
You thus want to play rock which grants `1` point.\
`3 + 1 = 4`\
On line 2 you want to lose `X`, and earn `0` point.\
Your opponent played paper `B`.\
You thus want to play rock which grants `1` point.\
`0 + 1 = 1`\
On line 3 you want to win `Z`, and earn `6` points.\
Your opponent played scissors `C`.\
You thus want to play rock which grants `1` point.\
`6 + 1 = 7`\

Your final score is `4 + 1 + 7 = 12`.
