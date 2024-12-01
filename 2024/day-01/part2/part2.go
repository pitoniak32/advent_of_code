package main

import (
	"fmt"
	"log"
	"slices"
	"strconv"
	"strings"

	"github.com/pitoniak32/advent_of_code/2024/util"
)

func main() {
	// Reused first input.
	inputFileContents := util.GetInputContents("input1.txt")

	result := FindSimilarityScore(string(inputFileContents))

	fmt.Println(result)
}

func FindSimilarityScore(input string) int {

	firstList, secondList := GetIdLists(string(input))
	fmt.Println(firstList)
	fmt.Println(secondList)
	firstLen := len(firstList)
	secondLen := len(secondList)
	if firstLen != secondLen {
		log.Fatalf("lists do not have the same length (%d != %d). Cannot continue", firstLen, secondLen)
	}
	var total = 0
	for _, f := range firstList {
		occurances := 0
		for _, s := range secondList {
			if f == s {
				occurances += 1
			}
		}
		total += occurances * f
	}

	return total
}

func GetIdLists(input string) ([]int, []int) {
	firstRow, secondRow := []int{}, []int{}

	lines := strings.Split(input, "\n")
	for _, line := range lines {
		parts := strings.Split(line, "   ")
		if len(parts) != 2 {
			log.Fatalf("each line should contain 2 items 3 spaces apart. Got %s", line)
		}
		fmt.Printf("l: %s, r: %s\n", parts[0], parts[1])
		first, err := strconv.Atoi(parts[0])
		if err != nil {
			log.Fatalf("first part could not be converted into integer: %v", err)
		}
		second, err := strconv.Atoi(parts[1])
		if err != nil {
			log.Fatalf("second part could not be converted into integer: %v", err)
		}
		firstRow = append(firstRow, first)
		secondRow = append(secondRow, second)
	}

	slices.Sort(firstRow)
	slices.Sort(secondRow)

	return firstRow, secondRow
}
