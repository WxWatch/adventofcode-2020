package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"sort"
	"strconv"
	"strings"
	"time"
)

func timeTrack(start time.Time, name string) {
	elapsed := time.Since(start)
	log.Printf("%s took %s", name, elapsed)
}

func getInput() []int {
	data, err := ioutil.ReadFile("input")
	if err != nil {
		log.Fatal(err)
	}

	stringEntries := strings.Fields(string(data))
	entries := make([]int, 0)
	for _, entry := range stringEntries {
		entryInt, _ := strconv.Atoi(entry)
		entries = append(entries, entryInt)
	}
	return entries
}

/* Naive solution; gets worse the bigger the initial entries array is */
func part1Iteration(entries []int) int {
	defer timeTrack(time.Now(), "part1Iteration")
	sort.Ints(entries)

	for i := 0; i < len(entries); i++ {
		for j := 0; j < len(entries); j++ {
			if i == j {
				continue
			}

			outerEntry := entries[i]
			innerEntry := entries[j]
			if innerEntry+outerEntry == 2020 {
				return innerEntry * outerEntry
			}
		}
	}

	return -1
}

/* Slower than part1Iteration */
func part1SortBinaryIteration(entries []int) int {
	defer timeTrack(time.Now(), "part1Sort")
	sort.Ints(entries)

	for _, entry := range entries {
		i := sort.Search(len(entries), func(i int) bool {
			return entry+entries[i] >= 2020
		})
		if i < len(entries) && entry+entries[i] == 2020 {
			// x is present at data[i]
			return entry * entries[i]
		}
	}

	return -1
}

/* Naive solution; gets worse the bigger the initial entries array is */
func part2Iteration(entries []int) int {
	defer timeTrack(time.Now(), "part2Iteration")
	sort.Ints(entries)

	for i := 0; i < len(entries); i++ {
		for j := 0; j < len(entries); j++ {
			for k := 0; k < len(entries); k++ {

				if i == j || j == k || i == k {
					continue
				}

				outerEntry := entries[i]
				midEntry := entries[j]
				innerEntry := entries[k]
				if innerEntry+midEntry+outerEntry == 2020 {
					return innerEntry * midEntry * outerEntry
				}
			}
		}
	}

	return -1
}

func main() {
	/* Notes:
	 * I sort first in every solution to get a more accurate indication of how long each function takes
	 */
	entries := getInput()
	fmt.Println(part1Iteration(entries))

	entries = getInput()
	fmt.Println(part2Iteration(entries))

	entries = getInput()
	fmt.Println(part1SortBinaryIteration(entries))
}
