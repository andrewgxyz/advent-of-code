package day02_test

import (
	"testing"

	"advent-of-code-2024/day02"
	"advent-of-code-2024/utils"
)

// func TestPartOne(t *testing.T) {
//   file := utils.LoadFile("./input.txt")
//   num := day02.PartOne(file)

//   if num != 2 {
//     t.Fatalf(`Part One: Expected 2 but got %v`, num)
//   }
// }

func TestPartTwo(t *testing.T) {
  file := utils.LoadFile("./input.txt")
  num := day02.PartTwo(file)

  if num != 4 {
    t.Fatalf(`Part Two: Expected 4 but got %v`, num)
  }
}

