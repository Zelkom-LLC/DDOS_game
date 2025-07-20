package main

import (
	"log"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"

	"attacker/game"
)

func main() {
	// загружаем .env (если не найден — просто логируем и идём дальше)
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found:", err)
	}

	// создаём роутер с дефолтными middleware (логирование, recovery)
	r := gin.Default()

	// корневой маршрут
	r.GET("/", func(c *gin.Context) {
		c.String(http.StatusOK, "Hello, Gin!")
	})

	// health-check маршрут
	r.GET("/health", health)

	r.GET("/ready", func(c *gin.Context) {
		c.String(http.StatusOK, "Ready!")
	})

	r.POST("/defense/:attack", defense)

	// запускаем сервер на 0.0.0.0:3000
	if err := r.Run(":8000"); err != nil {
		log.Fatalf("Ошибка запуска сервера: %v", err)
	}
}

// handler для /health
func health(c *gin.Context) {
	c.Status(http.StatusOK)
}

func defense(c *gin.Context) {
	attackStr := c.Param("attack")
	attack, err := strconv.ParseUint(attackStr, 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid number"})
		return
	}

	result := game.Fibonacci_iterative(attack)
	c.String(http.StatusOK, "Defense! - %d", result)
}
