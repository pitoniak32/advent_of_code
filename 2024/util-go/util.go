package util

import (
	"fmt"
	"log"
	"os"
)

func GetInputContents(fileName string) []byte {
	inputFileContents, err := os.ReadFile(fileName)
	if err != nil {
		log.Fatalf("unable to read file: %v", err)
	}
	return inputFileContents
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

/* func PrettyPrint(matrix [][]int) { */
/*     b, err := json.MarshalIndent(matrix, "", "  ") */
/*     if err != nil { */
/*         fmt.Println(err) */
/*         return */
/*     } */
/*     fmt.Println(string(b)) */
/* } */

func PrettyPrint(matrix [][]int) {
	for _, row := range matrix {
		for _, val := range row {
			fmt.Printf("%2d ", val)
		}
		fmt.Println()
	}
}
