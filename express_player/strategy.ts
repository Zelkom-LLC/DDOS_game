// Fibonacci recursive function
export function fibonacciRec(n: number): number {
    switch (n) {
        case 0:
            return 0;
        case 1:
            return 1;
    }
    return fibonacciRec(n - 1) + fibonacciRec(n - 2);
}

// Fibonacci iterative function
export function fibonacciIter(n: number): number {
    switch (n) {
        case 0:
            return 0;
        case 1:
            return 1;
    }
    let prev = 0;
    let curr = 1;
    for (let i = 2; i <= n; i++) {
        const next = prev + curr;
        prev = curr;
        curr = next;
    }
    return curr;
}

// CPU burn function
export function burnCPU(iterations: number): number {
    let x = 0.5;
    const r = 3.9999999;
    for (let i = 0; i < iterations; i++) {
        x = r * x * (1 - x);
    }
    return x;
}
