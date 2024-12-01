package day04

import (
	"bufio"
	"os"
	"strings"

	"advent-of-code-2021/utils"
)

func PartOne(file *os.File) int {
  scan := bufio.NewScanner(file)
  lines := utils.LoadLinesToString(scan)

  finalNum := 0
  bingoNumbers := strings.Split(lines[0], ",")
  var bingoBoards [][]string


  // Generate all the boards


  // Loop through each number til a victory
  for _, num := range bingoNumbers {

    for key, board := range bingoBoards {
      for x, val := range board {

      }
    }
  }


  return finalNum
}

// func PartTwo(file *os.File) int64 {
//   scan := bufio.NewScanner(file)
//   lines := utils.LoadLinesToString(scan)

//   ox, _ := strconv.ParseInt(OxygenRating(lines, 0), 2, 64)
//   co, _ := strconv.ParseInt(CarbonRating(lines, 0), 2, 64)

//   return ox * co
// }


func main() int {
  file := utils.LoadFile("./input.txt")
  defer file.Close()

  return PartOne(file)
  // return PartTwo(file)
}
