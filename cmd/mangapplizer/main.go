package main

import (
	"log"
	mangarock "mangapplizer-backend/pkg/parser/mangarock"
	"strconv"
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
	// TODO: fetch serie id and chapter id dynamically.
	chapter, e := mr.Chapter("mrs-serie-243382", "mrs-chapter-200100701")
	if e != nil {
		log.Fatal(e)
		return
	}

	// TODO: put decent structure for path
	// basePath + mangaName + language(like en/jp) + chapterOrder/
	path := "/tmp/" + strconv.Itoa(chapter.Order) + "/"

	err := mangarock.SaveChapter(&chapter, path)
	if err != nil {
		panic(err)
	}

	err = mangarock.ConvertMRItoPNG(path)
	if err != nil {
		panic(err)
	}

}
