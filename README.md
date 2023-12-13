# Advent_of_code_2023

My solutions to the Advent Of Code 2023 in Rust

## Day 7
To run the code I have not bothered with having a better type system to take care for both the joker and non-joker cases
So we need to manually change the `ORDER` variable from `23456789TJQKA` to `J23456789TQKA` to get the correct answer for part 2

## Day 10
For the case of S In my test case it is quite visible that S can only be '|' to allow for a loop to form  
So the code might need to be changed when calculating neighbours on what to do with character S in get\_nearby function
