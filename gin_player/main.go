package main

import (
    "github.com/gin-gonic/gin"
)

func main() {
    r := gin.Default() // создаём роутер с набором middleware по умолчанию

    r.GET("/", func(c *gin.Context) {
        c.JSON(200, gin.H{
            "message": "Hello, Gin!",
        })
    })

    r.Run(":8080") // запускаем сервер на порту 8080
}