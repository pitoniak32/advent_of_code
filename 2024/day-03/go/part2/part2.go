package main

import (
	"fmt"
	"log"
	"regexp"
	"strconv"
	"strings"

	"github.com/pitoniak32/advent_of_code/2024/util-go"
)

func main() {
	inputFileContents := util.GetInputContents("day-03/input1.txt")
  result := ProcessMulInstructions(string(inputFileContents))
  fmt.Println(result)
}

func ProcessMulInstructions(input string) int { 
	r, _ := regexp.Compile("mul\\([0-9]{1,3},[0-9]{1,3}\\)|do\\(\\)|don\\'t\\(\\)")
  allInstructions := r.FindAllString(input, -1)
  fmt.Println(allInstructions)
  total := 0
  stopped := false
  for _, instruction := range allInstructions {
    if instruction == "don't()" {
      stopped = true
      continue
    }
    if instruction == "do()" {
      stopped = false
      continue
    }

    if !stopped {
      trimmed := strings.Trim(instruction, "mul()")
      parts := strings.Split(trimmed, ",")
      if len(parts) != 2 {
        panic("the mul instructions should only have 2 operands")
      }

      first, err := strconv.Atoi(parts[0])
      if err != nil {
        log.Fatalf("part %s could not be converted into integer: %v", parts[0], err)
      }
      second, err := strconv.Atoi(parts[1])
      if err != nil {
        log.Fatalf("part %s could not be converted into integer: %v", parts[1], err)
      }
      total += first * second
    }
  }

  return total
}
