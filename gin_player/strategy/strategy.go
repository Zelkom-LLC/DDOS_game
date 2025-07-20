package strategy

func Fibonacci(n uint64) uint64 {
	switch n {
	case 0:
		return 0
	case 1:
		return 1
	}
	return Fibonacci(n-1) + Fibonacci(n-2)
}

func FibonacciIterative(n uint64) uint64 {
	switch n {
	case 0:
		return 0
	case 1:
		return 1
	}

	var prev, curr uint64 = 0, 1
	for i := uint64(2); i <= n; i++ {
		next := prev + curr
		prev = curr
		curr = next
	}
	return curr
}

func BurnCPU(iterations int) float64 {
	x := 0.5
	r := 3.9999999
	for i := 0; i < iterations; i++ {
		x = r * x * (1.0 - x)
	}
	return x
}
