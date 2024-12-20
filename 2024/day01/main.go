package day01

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"

	"advent-of-code-2024/utils"
)

func PartOne(file *os.File) int {
  scan := bufio.NewScanner(file)

  lines := utils.LoadLinesToString(scan)
  var left []int
  var right []int
  total := 0

  for _, line := range lines {
    nums := strings.Split(line, "   ")

    num1, _ := strconv.Atoi(nums[0]) 
    num2, _ := strconv.Atoi(nums[1]) 

    left = append(left, num1)
    right = append(right, num2)
  }

  sort.Ints(left)
  sort.Ints(right)

  fmt.Println(left)
  fmt.Println(right)

  for key, num := range left {
    diff := right[key] - num

    fmt.Println(diff)

    if diff < 0 {
      diff = diff * -1
    }

    total += diff
  }

  return total
}

func PartTwo(file *os.File) int {
  scan := bufio.NewScanner(file)
  lines := utils.LoadLinesToString(scan)
  rightTotal := make(map[int]int)
  var left []int
  total := 0

  for _, line := range lines {
    nums := strings.Split(line, "   ")

    num1, _ := strconv.Atoi(nums[0]) 
    num2, _ := strconv.Atoi(nums[1]) 

    _, ok := rightTotal[num2]

    if !ok {
      rightTotal[num2] = 1
    } else {
      rightTotal[num2] += 1
    }

    left = append(left, num1)
  }

  for _, num := range left {
    fmt.Println(rightTotal[num] * num)
    total += rightTotal[num] * num
  }

  return total
}


func main() int {
  file := utils.LoadFile("./input.txt")
  defer file.Close()

  return PartOne(file)
  // return PartTwo(file)
}
