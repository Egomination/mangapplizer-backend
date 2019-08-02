package main

import (
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

	ids, err := mr.Search("bleach")
	if err != nil {
		panic(err)
	}

	series, err := mr.Mangas(ids)
	if err != nil {
		panic(err)
	}
	// s stands for the one series
	for _, s := range series {
		manga, _ := mr.Manga(s.ID)
		// Emulating user selected the 0th chapter which is the chapter 1
		chapter := manga.Chapters[0]
		pages, _ := mr.Chapter(manga.ID, chapter.ID)
		log.Printf("%s", chapter)
		path := "/tmp/" + mangarock.NormalizeOneDigitNumber(chapter.Order) + "-" +
			chapter.ID + "/"

		err := mangarock.SaveChapter(&pages, path)
		if err != nil {
			panic(err)
		}

		err = mangarock.ConvertMRItoPNG(path)
		if err != nil {
			panic(err)
		}
	}
}
