package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

func getInput() []string {
	data, err := ioutil.ReadFile("input")
	if err != nil {
		log.Fatal(err)
	}

	return strings.Fields(string(data))
}

/* Naive solution; gets worse the bigger the initial entries array is */
func part1Iteration(entries []string) int {
	for i := 0; i < len(entries); i++ {
		for j := 0; j < len(entries); j++ {
			if i == j {
				continue
			}

			outerEntry, _ := strconv.Atoi(entries[i])
			innerEntry, _ := strconv.Atoi(entries[j])
			if innerEntry+outerEntry == 2020 {
				return innerEntry * outerEntry
			}
		}
	}

	return -1
}

/* Naive solution; gets worse the bigger the initial entries array is */
func part2Iteration(entries []string) int {
	for i := 0; i < len(entries); i++ {
		for j := 0; j < len(entries); j++ {
			for k := 0; k < len(entries); k++ {
				if i == j || j == k || i == k {
					continue
				}

				outerEntry, _ := strconv.Atoi(entries[i])
				midEntry, _ := strconv.Atoi(entries[j])
				innerEntry, _ := strconv.Atoi(entries[k])
				if innerEntry+midEntry+outerEntry == 2020 {
					return innerEntry * midEntry * outerEntry
				}
			}
		}
	}

	return -1
}

func main() {
	entries := getInput()
	fmt.Println(part1Iteration(entries))
	fmt.Println(part2Iteration(entries))
}
