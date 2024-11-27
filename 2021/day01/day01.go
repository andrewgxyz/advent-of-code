package day01

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func PartOne(nums []int) int {
  incrementCount := 0
  currentTotal := 0

  for _, num := range nums {
    if num > currentTotal && currentTotal != 0 {
      incrementCount += 1
    }

    currentTotal = num
  }

  return incrementCount
}

func PartTwo(nums []int) int {
  var newNums []int

  for i := range nums {
    if (i + 3) > len(nums) {
      break;
    }

    num := 0
    for e := i; e < (i + 3); e++ {
      num += nums[e]
    }

    newNums = append(newNums, num)
  }

  fmt.Println(newNums)
  fmt.Println(nums)

  return PartOne(newNums)
}

func LoadFile(filename string) *os.File {
  file, err := os.Open(filename)

  if err != nil {
    fmt.Println(err)
  }

  defer file.Close()

  return file
}

func Dayone() {
  file, err := os.Open("./input.txt")
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

  fmt.Println(PartOne(nums))
  fmt.Println(PartTwo(nums))
}
