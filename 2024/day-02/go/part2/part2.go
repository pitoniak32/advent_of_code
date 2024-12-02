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

	safeReports := 0

	for row, report := range reports {
		safe := CheckReportSafety(report)

		if safe {
			safeReports += 1
		} else {
			fmt.Printf("found unsafe report at [%d]. Checking if error correction helps\n\n", row)
			safeWithRemoved := false
			for i, _ := range report {
				modifiedReport := make([]int, len(report))
				copy(modifiedReport, report)
				fmt.Printf("report      : %v\n", report)
				fmt.Printf("report[:i]  : %v\n", report[:i])
				fmt.Printf("report[i+1:]: %v\n", report[i+1:])
				fmt.Printf("removed: report[%d] = %d\n", i, report[i])
				modifiedReport = append(modifiedReport[:i], modifiedReport[i+1:]...)
				fmt.Printf("mod: %v\n", modifiedReport)
				if CheckReportSafety(modifiedReport) {
					fmt.Println("found safe with removed!")
					safeWithRemoved = true
					continue
				}
			}
			if safeWithRemoved {
				safeReports += 1
				fmt.Printf("Finished check. Report is safe!\n\n")
			} else {
				fmt.Printf("Finished check. Report is still unsafe\n\n")
			}
		}
	}

	return safeReports
}

func CheckReportSafety(report []int) bool {
	safe := true
	negCount := 0
	posCount := 0
	for i := 0; i < len(report)-1; i++ {
		cur := report[i]
		next := report[i+1]

		diff := cur - next

		if diff < 0 {
			negCount += 1
		} else {
			posCount += 1
		}

		absDiff := util.Abs(diff)

		if absDiff < 1 || absDiff > 3 {
			safe = false
		}
		fmt.Printf("first: %d, next: %d, diff: %d, absDiff: %d, safe: %v\n", cur, next, diff, absDiff, safe)
	}
	fmt.Printf("safe: %v, pos: %d, neg: %d\n\n", safe, posCount, negCount)
	return safe && ((posCount > 0 && negCount == 0) || (posCount == 0 && negCount > 0))
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
