package day02

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"advent-of-code-2024/utils"
)

func PartOne(file *os.File) int {
  scan := bufio.NewScanner(file)

  lines := utils.LoadLinesToString(scan)
  safe := 0

  for _, line := range lines {
    var nums []int

    strNums := strings.Split(line, " ")
    isSafe := true
    isDecrease := true

    for _, n := range strNums {
      num, _ := strconv.Atoi(n)
      nums = append(nums, num)
    }

    prev := 0

    for _, n := range nums {
      if prev != 0 {
        diff := n - prev

        if diff > 3 || diff < -3 || diff == 0 {
          isSafe = false
        }

        if diff > 0 && isDecrease || diff < 0 && !isDecrease {
          isSafe = false
        }

        fmt.Println(diff, isDecrease)
      } else {
        isDecrease = nums[1] - n < 0
        fmt.Println(isDecrease)
      }

      prev = n
    }

    fmt.Println(isSafe)
    fmt.Println("")
    if isSafe {
      safe += 1
    }
  }

  return safe
}

func PartTwo(file *os.File) int {
  scan := bufio.NewScanner(file)

  lines := utils.LoadLinesToString(scan)
  safe := 0

  for _, line := range lines {
    var nums []int
    var diffs []int

    strNums := strings.Split(line, " ")
    isSafe := true

    for _, n := range strNums {
      num, _ := strconv.Atoi(n)
      nums = append(nums, num)
    }

    prev := 0

    for _, n := range nums {
      if prev != 0 {
        diff := n - prev
        diffs = append(diffs, diff)
      } else {
        diffs = append(diffs, nums[1] - n)
      }

      prev = n
    }

    isDecrease := false
    unsafeCount := 0

    for key, d := range diffs {
      if key == 0 {
        isDecrease = d < 0
      } else {
        if (d > 0 && isDecrease) || (d < 0 && !isDecrease) {
          unsafeCount += 1
        } else if d > 3 || d < -3 || d == 0 {
          isSafe = false
          unsafeCount += 1
        }

        if unsafeCount > 1 {
          isSafe = false
        }
      }
    }


    if isSafe {
      safe += 1
    } else {
      fmt.Println(diffs)
      fmt.Println(isDecrease)
      fmt.Println(unsafeCount)
      fmt.Println(isSafe)
      fmt.Println("")
      fmt.Println("")
    }
  }

  return safe
}


func main() int {
  file := utils.LoadFile("./input.txt")
  defer file.Close()

  return PartOne(file)
  // return PartTwo(file)
}
