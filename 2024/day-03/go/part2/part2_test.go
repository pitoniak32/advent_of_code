package main

import (
	"fmt"
	"testing"
)

func TestPart2(t *testing.T) {
  expected := 48
  input := `xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`
  result := ProcessMulInstructions(input)
  fmt.Printf("%v", result)
  if result != expected {
    t.Errorf("Expected %d, got %d", expected, result)
  }
}

