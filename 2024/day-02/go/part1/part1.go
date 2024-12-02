package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.com/pitoniak32/advent_of_code/2024/util-go"
)

func main() {
	inputFileContents := util.GetInputContents("day-02/input1.txt")
  result := CountSafeReports(string(inputFileContents))
  fmt.Printf("result: %d", result)
}


func CountSafeReports(input string) int {
  reports := GetReports(input)

  result := 0

  for _, report := range reports {
    safe := true
    negCount := 0
    posCount := 0
    for i := 0; i < len(report)-1; i++ {
      first := report[i]
      next := report[i+1]

      diff := first - next

      if diff < 0 {
        negCount += 1
      } else {
        posCount += 1
      }

      absDiff := util.Abs(diff)

      if absDiff < 1 || absDiff > 3 {
        safe = false
      }
      fmt.Printf("first: %d, next: %d, diff: %d, absDiff: %d, safe: %v\n", first, next, diff, absDiff, safe)
    }
    fmt.Printf("safe: %v, pos: %d, neg: %d\n\n", safe, posCount, negCount)
    if safe && ((posCount > 0 && negCount == 0) || (posCount == 0 && negCount > 0)) {
      result += 1
    }
  }

  return result
}

func GetReports(input string) [][]int {
	reports := [][]int{}

	lines := strings.Split(input, "\n")
	for row, line := range lines {
    if strings.Trim(line, "\n ") != "" {
      parts := strings.Split(line, " ")
      reportLine := []int{}
      for col, part := range parts {
        num, err := strconv.Atoi(part)
        if err != nil {
          log.Fatalf("part '%s' at row = %d, col = %d, in line = '%s' could not be converted into integer: %v", part, row, col, line, err)
        }
        reportLine = append(reportLine, num)
      }
      reports = append(reports, reportLine)
    }
	}

	return reports
}
