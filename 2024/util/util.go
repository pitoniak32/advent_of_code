package util

import (
	"log"
	"os"
	"path"
)

func GetInputContents(fileName string) []byte {
	args := os.Args
	if len(args) != 2 {
		log.Fatalf("Invalid number of arguments! Expected 2, got %d.", len(args))
	}
	curDay := args[1]
	inputFileContents, err := os.ReadFile(path.Join(curDay, fileName))
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
