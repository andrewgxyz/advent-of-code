package day04

import (
	"bufio"
	"os"

	"advent-of-code-2024/utils"
)

func PartOne(file *os.File) int {
  scan := bufio.NewScanner(file)
  grid := utils.LoadStringToGrid(scan)
  total := 0
  word := []string{"X", "M", "A", "S"}
  // neighbors := [][]int{
  //   []int{0, 1},
  //   []int{0, -1},
  // }

  for y, line := range grid {
    for x, col := range line {
      var neighbors [][]int

      if y > 0 {
        neighbors = append(neighbors, []int{-1, 0})
        if x > 0 {
          neighbors = append(neighbors, []int{-1, -1})
        }

        if x < (len(line) - 1) {
          neighbors = append(neighbors, []int{-1, 1})
        }
      }

      if y < (len(grid) - 1) {
        neighbors = append(neighbors, []int{1, 0})

        if x > 0 {
          neighbors = append(neighbors, []int{1, -1})
        }

        if x < (len(line) - 1) {
          neighbors = append(neighbors, []int{1, 1})
        }
      }

      if x > 0 {
        neighbors = append(neighbors, []int{0, -1})
      }

      if x < (len(line) - 1) {
        neighbors = append(neighbors, []int{0, 1})
      }

      if col == word[0] {
        for _, direction := range neighbors {
          if grid[y + direction[0]][x + direction[0]] == word[1] {
            if grid[y + (direction[0]*2)][x + (direction[1]*2)] == word[2] {
              if grid[y + (direction[0]*3)][x + (direction[1]*3)] == word[3] {
                total += 1
              }
            }
          }
        }
      }
    }
  }

  return total
}

// func PartTwo(file *os.File) int {
//   scan := bufio.NewScanner(file)
//   lines := utils.LoadLinesToString(scan)
//   total := 0
//   d, _ := regexp.Compile("do\\(\\)")
//   dn, _ := regexp.Compile("don\\'t\\(\\)")
//   r, _ := regexp.Compile("mul\\(\\d*,\\d*\\)")

//   doit := true
//   for _, line := range lines {
//     does := d.FindAllStringIndex(line, -1)
//     donts := dn.FindAllStringIndex(line, -1)
//     runs := r.FindAllStringIndex(line, -1)

//     if len(does) > 0 && len(donts) > 0 {
//       if donts[0][0] > does[0][0] {
//         doit = false
//       } else {
//         doit = true
//       }
//     } else if len(does) > 0 {
//       doit = true
//     } else if len(donts) > 0 {
//       if donts[0][0] > runs[0][0] {
//         doit = true
//       } else {
//         doit = false
//       }
//     }

//     // fmt.Println(does)
//     // fmt.Println(donts)
//     // fmt.Println(runs)

//     multiples := r.FindAllString(line, -1)

//     for _, mul := range multiples {
//       if doit {
//         mul = strings.Replace(mul, "mul(", "", -1)
//         mul = strings.Replace(mul, ")", "", -1)

//         // fmt.Println(mul)
//         nums := strings.Split(mul, ",")

//         num0, _ := strconv.Atoi(nums[0])
//         num1, _ := strconv.Atoi(nums[1])

//         total += num0 * num1
//       }
//     }
//   }

//   return total
// }


func main() int {
  file := utils.LoadFile("./input.txt")
  defer file.Close()

  return PartOne(file)
  // return PartTwo(file)
}
