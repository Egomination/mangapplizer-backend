package main

import (
	"fmt"
	anilist "mangapplizer-backend/pkg/anilist"
)

func main() {
	resp, _ := anilist.PostPayload()
	fmt.Println(resp)
}
