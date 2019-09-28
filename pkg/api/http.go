package api

import (
	"log"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
)

func setupRouter(username, password string) *gin.Engine {
	hostname := "Test"

	gin.SetMode(gin.ReleaseMode)

	r := gin.New()

	r.Use(gin.Recovery())
	r.Use(addHostname(hostname))

	// authorized := r.Group("/", ginBasicAuth(username, password))
	authorized := r.Group("/", ginBasicAuth(username, password))

	v1 := authorized.Group("/api/v1")
	{
		v1Handlers := NewAPIHandler()

		v1.GET("/", v1Handlers.root)
		v1.GET("/search/:manga", v1Handlers.getManga)
	}

	return r

}

// RunServer is the driver function which triggers the listeners
func RunServer(httpListen, username, password string) {
	r := setupRouter(username, password)

	srv := &http.Server{
		Addr:         httpListen,
		Handler:      r,
		ReadTimeout:  60 * time.Second,
		WriteTimeout: 60 * time.Second,
	}

	go func() {
		log.Println("Listening http address: ", srv.Addr)
		if err := srv.ListenAndServe(); err != nil {
			log.Fatal("Error!")
		}
	}()
}

func addHostname(name string) gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Header("X-Name", name)
		c.Next()
	}
}

func ginBasicAuth(username, password string) gin.HandlerFunc {
	return gin.BasicAuth(gin.Accounts{username: password})
}
