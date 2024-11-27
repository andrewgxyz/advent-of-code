package day02_test

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"testing"

	"advent-of-code-2021/day02"
)

func TestPartOne(t *testing.T) {
  file, err := os.Open("./input_test.txt")
  directions := make(map[string]int)

  if err != nil {
    fmt.Println(err)
  }

  defer file.Close()

  scan := bufio.NewScanner(file)

  for scan.Scan() {
    line := strings.Split(scan.Text(), " ")

    num, err := strconv.Atoi(line[0])

    if err != nil {
      fmt.Println(err)
    }

    directions[line[0]] += num
  }

  num := day02.PartOne(directions)
  if num != 7 {
    t.Fatalf(`Part One: Expected 7 but got %v`, num)
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
