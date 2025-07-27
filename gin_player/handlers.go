package main

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

// Обёртка для рекурсивного фибоначчи
func FibonacciHandler(c *gin.Context) {
	iterStr := c.Param("iter")
	n, err := strconv.ParseUint(iterStr, 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid number"})
		return
	}
	result := Fibonacci(n)
	c.JSON(http.StatusOK, gin.H{"result": result})
}

// Обёртка для итеративного фибоначчи
func FibonacciIterativeHandler(c *gin.Context) {
	iterStr := c.Param("iter")
	n, err := strconv.ParseUint(iterStr, 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid number"})
		return
	}
	result := FibonacciIterative(n)
	c.JSON(http.StatusOK, gin.H{"result": result})
}

// Обёртка для нагрузки на CPU
func BurnCPUHandler(c *gin.Context) {
	iterStr := c.Param("iter")
	n, err := strconv.Atoi(iterStr)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid number"})
		return
	}
	result := BurnCPU(n)
	c.JSON(http.StatusOK, gin.H{"result": result})
}
