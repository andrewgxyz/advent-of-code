package day01_test

import (
	"testing"

	"advent-of-code-2024/day01"
	"advent-of-code-2024/utils"
)

func TestPartOne(t *testing.T) {
  file := utils.LoadFile("./input_test.txt")
  num := day01.PartOne(file)

  if num != 11 {
    t.Fatalf(`Part One: Expected 11 but got %v`, num)
  }
}

func TestPartTwo(t *testing.T) {
  file := utils.LoadFile("./input.txt")
  num := day01.PartTwo(file)

  if num != 31 {
    t.Fatalf(`Part Two: Expected 31 but got %v`, num)
  }
}

// func TestPartTwo(t *testing.T) {
//   file, err := os.Open("./input_test.txt")
//   var nums []int

//   if err != nil {
//     fmt.Println(err)
//   }

//   defer file.Close()

//   scan := bufio.NewScanner(file)

//   for scan.Scan() {
//     num, err := strconv.Atoi(scan.Text())
//     if err != nil {
//       fmt.Println(err)
//     }

//     nums = append(nums, num)
//   }

//   numPartTwo := day01.PartTwo(nums)
//   if numPartTwo != 5 {
//     t.Fatalf(`Part Two: Expected 5 but got %v`, numPartTwo)
//   }
// }
