package utils

func ArraySum(numbers []int) int {
	sum := 0
	for _, num := range numbers {
		sum += num
	}

	return sum
}
