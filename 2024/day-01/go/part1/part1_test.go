package main

import "testing"

func TestFindDistanceSum(t *testing.T) {
	input := `3   4
4   3
2   5
1   3
3   9
3   3`

	result := FindDistanceSum(input)

	if result != 11 {
		t.Errorf("Expected 11, got %d", result)
	}
}
