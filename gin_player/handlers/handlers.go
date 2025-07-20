package handlers

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"

	"attacker/strategy"
)

// Обёртка для рекурсивного фибоначчи
func FibonacciHandler(c *gin.Context) {
	iterStr := c.Param("iter")
	n, err := strconv.ParseUint(iterStr, 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid number"})
		return
	}
	result := strategy.Fibonacci(n)
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
	result := strategy.FibonacciIterative(n)
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
	result := strategy.BurnCPU(n)
	c.JSON(http.StatusOK, gin.H{"result": result})
}
