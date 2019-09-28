package api

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

// HandlerV1 is the main object that holds api/v1 related params
// for no I don't know what to put so just created it in advance
type HandlerV1 struct{}

// NewAPIHandler returns to api handler struct pointer
func NewAPIHandler() *HandlerV1 {
	return &HandlerV1{}
}

func (handler *HandlerV1) getManga(c *gin.Context) {
	mangaName := c.Param("manga")

	c.JSON(http.StatusOK, gin.H{"status": "ok", "body": mangaName})
}

func (handler *HandlerV1) root(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{"status": "ok", "body": "Welcome!"})
}
