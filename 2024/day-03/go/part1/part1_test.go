package main

import (
	"fmt"
	"testing"
)

func TestPart1(t *testing.T) {
  expected := 161
  input := `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`
  result := ProcessMulInstructions(input)
  fmt.Printf("%v", result)
  if result != expected {
    t.Errorf("Expected %d, got %d", expected, result)
  }
}
