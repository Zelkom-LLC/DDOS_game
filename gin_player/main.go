package main

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"

    "attacker/handlers"
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

	r.POST("/start", start)

	r.POST("/fib_rec/:iter", handlers.FibonacciHandler)
	r.POST("/fib_iter/:iter", handlers.FibonacciIterativeHandler)
	r.POST("/burn/:iter", handlers.BurnCPUHandler)

	// запускаем сервер на 0.0.0.0:3000
	if err := r.Run(":8000"); err != nil {
		log.Fatalf("Ошибка запуска сервера: %v", err)
	}
}

// handler для /health
func health(c *gin.Context) {
	c.Status(http.StatusOK)
}

func start(c *gin.Context) {
	var settings game.GameSettings

	if err := c.ShouldBindJSON(&settings); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request body"})
		return
	}

	log.Printf("Start the round with settings\nSettings: connections = %d, delay = %dms, round = %ds, targets = %v",
		settings.ConnectionsAmount, settings.DelayMs, settings.RoundSec, settings.Targets)

	go game.Start_attack_game(settings)

	c.Status(http.StatusOK)
}
