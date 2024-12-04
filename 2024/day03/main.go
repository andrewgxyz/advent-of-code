package day03

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"

	"advent-of-code-2024/utils"
)

func PartOne(file *os.File) int {
  scan := bufio.NewScanner(file)
  total := 0

  for scan.Scan() {
    line := scan.Text()

    r, _ := regexp.Compile("mul\\(\\d*,\\d*\\)")

    multiples := r.FindAllString(line, -1)

    fmt.Println(line)
    fmt.Println(multiples)

    for _, mul := range multiples {
      mul = strings.Replace(mul, "mul(", "", -1)
      mul = strings.Replace(mul, ")", "", -1)

      fmt.Println(mul)
      nums := strings.Split(mul, ",")

      num0, _ := strconv.Atoi(nums[0])
      num1, _ := strconv.Atoi(nums[1])

      total += num0 * num1
    }
  }

  return total
}

func PartTwo(file *os.File) int {
  scan := bufio.NewScanner(file)
  lines := utils.LoadLinesToString(scan)
  total := 0
  d, _ := regexp.Compile("do\\(\\)")
  dn, _ := regexp.Compile("don\\'t\\(\\)")
  r, _ := regexp.Compile("mul\\(\\d*,\\d*\\)")

  doit := true
  for _, line := range lines {
    does := d.FindAllStringIndex(line, -1)
    donts := dn.FindAllStringIndex(line, -1)
    runs := r.FindAllStringIndex(line, -1)

    if len(does) > 0 && len(donts) > 0 {
      if donts[0][0] > does[0][0] {
        doit = false
      } else {
        doit = true
      }
    } else if len(does) > 0 {
      doit = true
    } else if len(donts) > 0 {
      if donts[0][0] > runs[0][0] {
        doit = true
      } else {
        doit = false
      }
    }

    // fmt.Println(does)
    // fmt.Println(donts)
    // fmt.Println(runs)

    multiples := r.FindAllString(line, -1)

    for _, mul := range multiples {
      if doit {
        mul = strings.Replace(mul, "mul(", "", -1)
        mul = strings.Replace(mul, ")", "", -1)

        // fmt.Println(mul)
        nums := strings.Split(mul, ",")

        num0, _ := strconv.Atoi(nums[0])
        num1, _ := strconv.Atoi(nums[1])

        total += num0 * num1
      }
    }
  }

  return total
}


func main() int {
  file := utils.LoadFile("./input.txt")
  defer file.Close()

  return PartOne(file)
  // return PartTwo(file)
}
