package day02

import (
	"advent-of-code-2021/utils"
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

func PartOne(nums map[string]int) int {
  x := 0
  y := 0

  for key, num := range nums {
    if key == "up" {
      num = num * -1
    }

    if key == "forward" {
      x += num
    } else {
      y += num
    }
  }

  return y * x
}

func PartTwo(scan *bufio.Scanner) int {
  x   := 0
  y   := 0
  aim := 0

  for scan.Scan() {
    line := strings.Split(scan.Text(), " ")

    num, err := strconv.Atoi(line[1])

    if err != nil {
      fmt.Println(err)
    }

    fmt.Println(line)
    if line[0] == "up" {
      num = num * -1
    }

    if line[0] == "forward" {
      x += num
      y += aim * num
    } else {
      aim += num
    }
    fmt.Println(x, " ", y)
  }

  return y * x
}


func Daytwo() int {
  file := utils.LoadFile("./day02/input.txt")
  scan := bufio.NewScanner(file)

  defer file.Close()

  return PartTwo(scan)
  // fmt.Println(PartTwo(nums))
}
