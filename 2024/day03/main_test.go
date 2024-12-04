package day03_test

import (
	"testing"

	"advent-of-code-2024/day03"
	"advent-of-code-2024/utils"
)

func TestPartOne(t *testing.T) {
  file := utils.LoadFile("./input.txt")
  num := day03.PartOne(file)

  if num != 161 {
    t.Fatalf(`Part One: Expected 161 but got %v`, num)
  }
}

func TestPartTwo(t *testing.T) {
  file := utils.LoadFile("./input.txt")
  num := day03.PartTwo(file)

  if num != 48 {
    t.Fatalf(`Part Two: Expected 48 but got %v`, num)
  }
}

