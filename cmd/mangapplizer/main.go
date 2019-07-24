package main

import (
	"fmt"
	"log"
	mangarock "mangapplizer-backend/pkg/parser/mangarock"
)

const (
	anilistURL = "https://graphql.anilist.co/"
)

func main() {
	// q := anilist.New(anilistURL)
	// resp, _ := q.PostPayload()
	// fmt.Println(resp)

	// Testing mangarock
	options := make(map[string]string)
	options["country"] = "United States"
	mr := mangarock.New(mangarock.WithOptions(options))

	m, e := mr.Manga("mrs-serie-100177863")
	if e != nil {
		log.Fatal(e)
		return
	}
	fmt.Println(m)

}
