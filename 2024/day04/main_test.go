package day04_test

import (
	"testing"

	"advent-of-code-2024/day04"
	"advent-of-code-2024/utils"
)

func TestPartOne(t *testing.T) {
  file := utils.LoadFile("./input_test.txt")
  num := day04.PartOne(file)

  if num != 18 {
    t.Fatalf(`Part One: Expected 18 but got %v`, num)
  }
}

// func TestPartTwo(t *testing.T) {
//   file := utils.LoadFile("./input.txt")
//   num := day04.PartTwo(file)

//   if num != 48 {
//     t.Fatalf(`Part Two: Expected 48 but got %v`, num)
//   }
// }

