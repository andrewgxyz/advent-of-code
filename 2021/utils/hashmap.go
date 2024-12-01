package utils

import "sort"

func SortKeys (gamma map[int]int) []int {
  keys := []int{}

  for key, _ := range gamma {
    keys = append(keys, key)
  }

  sort.Ints(keys)

  return keys
}
