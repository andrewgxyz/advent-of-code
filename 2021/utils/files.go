package utils

import (
	"fmt"
	"os"
)

func LoadFile(filename string) *os.File {
  file, err := os.Open(filename)

  if err != nil {
    fmt.Println(err)
  }

  return file
}
