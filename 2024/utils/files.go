package utils

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)


func LoadLinesToString(buff *bufio.Scanner) []string {
  var lines []string
  for buff.Scan() {
    lines = append(lines, buff.Text())
  }

  return lines
}

func LoadStringToGrid(buff *bufio.Scanner) [][]string {
  var lines [][]string
  for buff.Scan() {
    lines = append(lines, strings.Split(buff.Text(), ""))
  }

  return lines
}

func LoadFile(filename string) *os.File {
  file, err := os.Open(filename)

  if err != nil {
    fmt.Println(err)
  }

  return file
}
