package day01_test

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"testing"

  "advent-of-code-2021/day01"
)

func TestPartOne(t *testing.T) {
  file, err := os.Open("./input_test.txt")
  var nums []int

  if err != nil {
    fmt.Println(err)
  }

  defer file.Close()

  scan := bufio.NewScanner(file)

  for scan.Scan() {
    num, err := strconv.Atoi(scan.Text())
    if err != nil {
      fmt.Println(err)
    }

    nums = append(nums, num)
  }

  numPartOne := day01.PartOne(nums)
  if numPartOne != 7 {
    t.Fatalf(`Part One: Expected 7 but got %v`, numPartOne)
  }
}

func TestPartTwo(t *testing.T) {
  file, err := os.Open("./input_test.txt")
  var nums []int

  if err != nil {
    fmt.Println(err)
  }

  defer file.Close()

  scan := bufio.NewScanner(file)

  for scan.Scan() {
    num, err := strconv.Atoi(scan.Text())
    if err != nil {
      fmt.Println(err)
    }

    nums = append(nums, num)
  }

  numPartTwo := day01.PartTwo(nums)
  if numPartTwo != 5 {
    t.Fatalf(`Part Two: Expected 5 but got %v`, numPartTwo)
  }
}
