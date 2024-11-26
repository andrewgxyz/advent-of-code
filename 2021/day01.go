package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func increaseCheck(nums []int) int {
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

func partTwo(nums []int) int {
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

  return increaseCheck(newNums)
}

func Dayone() int {
  file, err := os.Open("./dayone.txt")
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

  return partTwo(nums)
}
