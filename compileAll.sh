#!/bin/bash
(cd day1 && cargo test && cd ..) || exit 1
(cd day2 && cargo test && cd ..) || exit 1
(cd day3 && cargo test && cd ..) || exit 1
(cd day4 && cargo test && cd ..) || exit 1
(cd day5 && cargo test && cd ..) || exit 1
(cd day6 && cargo test && cd ..) || exit 1
(cd day7 && cargo test && cd ..) || exit 1
(cd day8 && cargo test && cd ..) || exit 1
(cd day9 && cargo test && cd ..) || exit 1
(cd day10 && cargo test && cd ..) || exit 1
(cd day11 && cargo test && cd ..) || exit 1
#(cd day12 && cargo test && cd ..) || exit 1
#(cd day13 && cargo test && cd ..) || exit 1
#(cd day14 && cargo test && cd ..) || exit 1