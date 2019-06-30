package main

import (
	"fmt"
	anilist "mangapplizer-backend/pkg/anilist"
)

const (
	anilistURL = "https://graphql.anilist.co/"
)

func main() {
	q := anilist.New(anilistURL)
	resp, _ := q.PostPayload()
	fmt.Println(resp)
}
