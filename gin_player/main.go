package main

import (
    "log"
    "net/http"

    "github.com/gin-gonic/gin"
    "github.com/joho/godotenv"
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

    r.GET("/ping", func(c *gin.Context) {
        c.String(http.StatusOK, "Pong!")
    })

    // запускаем сервер на 0.0.0.0:3000
    if err := r.Run(":8000"); err != nil {
        log.Fatalf("Ошибка запуска сервера: %v", err)
    }
}

// handler для /health
func health(c *gin.Context) {
    c.Status(http.StatusOK)
}