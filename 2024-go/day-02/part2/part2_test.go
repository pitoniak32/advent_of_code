package main

import "testing"

func TestPart2(t *testing.T) {
  expected := 4
  input := `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`
  result := CountSafeReports(input)
  if result != expected {
    t.Errorf("Expected %d, got %d", expected, result)
  }
}
