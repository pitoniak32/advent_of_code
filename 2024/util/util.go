package util

import (
	"log"
	"os"
)

func GetInputContents() []byte {
	args := os.Args
	if len(args) != 2 {
		log.Fatalf("Invalid number of arguments! Expected 2, got %d.", len(args))
	}
	inputFile := args[1]
	inputFileContents, err := os.ReadFile(inputFile)
	if err != nil {
		log.Fatalf("unable to read file: %v", err)
	}
	return inputFileContents
}
