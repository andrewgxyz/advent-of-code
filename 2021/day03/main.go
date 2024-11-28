package day03

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"

	"advent-of-code-2021/utils"
)

func LoadLinesToString(buff *bufio.Scanner) []string {
  var lines []string
  for buff.Scan() {
    lines = append(lines, buff.Text())
  }

  return lines
}

func ParseGamma(lines []string) (map[int]int, map[int]int) {
  zeroes := make(map[int]int)
  ones := make(map[int]int)

  for _, line := range lines {
    for key, val := range line {
      if val == '0' {
        zeroes[key] += 1
      } else if val == '1' {
        ones[key] += 1
      }
    }
  }

  return zeroes, ones
}

func GetGamma(zeroes map[int]int, ones map[int]int) string {
  num := []string{}

  fmt.Println(zeroes, ones)
  keys := SortKeys(zeroes)

  for _, key := range keys {
    fmt.Println(zeroes[key], " ", ones[key])
    if zeroes[key] > ones[key] {
      num = append(num, "0")
    } else {
      num = append(num, "1")
    }
  }

  return strings.Join(num, "")
}

func SortKeys (gamma map[int]int) []int {
  keys := []int{}

  for key, _ := range gamma {
    keys = append(keys, key)
  }

  sort.Ints(keys)

  return keys
}

func GetEpsilon(gamma string) string {
  num := ""

  for _, val := range gamma {
    if val == '0' {
      num += "1"
    } else if val == '1' {
      num += "0"
    }
  }
  return num
}

func PartOne(file *os.File) int64 {
  scan := bufio.NewScanner(file)
  lines := LoadLinesToString(scan)
  zeroes, ones := ParseGamma(lines)

  gamma := GetGamma(zeroes, ones)
  epsilon := GetEpsilon(gamma)

  gammaNum, _   := strconv.ParseInt(gamma, 2, 64)
  epsilonNum, _ := strconv.ParseInt(epsilon, 2, 64)

  fmt.Println(gamma, " ", epsilon)
  fmt.Println(gammaNum, " ", epsilonNum)

  return gammaNum * epsilonNum
}

func OxygenRating(lines []string, column int) string {
  if column + 1 > len(lines[0]) || len(lines) == 1 {
    return lines[0]
  }
  var newLines []string
  zeroes, ones := ParseGamma(lines)
  var common byte

  if zeroes[column] > ones[column] {
    common = '0'
  } else {
    common = '1'
  }

  for _, line := range lines {
    if line[column] == common {
      newLines = append(newLines, line)
    }
  }

  fmt.Println(newLines)

  return OxygenRating(newLines, column+1)
}

func CarbonRating(lines []string, column int) string {
  if column + 1 > len(lines[0]) || len(lines) == 1 {
    return lines[0]
  }
  var newLines []string
  zeroes, ones := ParseGamma(lines)
  var common byte

  if ones[column] < zeroes[column] {
    common = '1'
  } else {
    common = '0'
  }

  for _, line := range lines {
    if line[column] == common {
      newLines = append(newLines, line)
    }
  }

  fmt.Println(newLines)

  return CarbonRating(newLines, column+1)
}

func PartTwo(file *os.File) int64 {
  scan := bufio.NewScanner(file)
  lines := LoadLinesToString(scan)

  ox, _ := strconv.ParseInt(OxygenRating(lines, 0), 2, 64)
  co, _ := strconv.ParseInt(CarbonRating(lines, 0), 2, 64)
  // co := OxygenRating(lines, 0)

  return ox * co
}


func main() int64 {
  file := utils.LoadFile("./input.txt")
  defer file.Close()

  return PartOne(file)
  // return PartTwo(file)
}
