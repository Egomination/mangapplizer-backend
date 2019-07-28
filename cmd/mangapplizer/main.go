package main

import (
	"fmt"
	"image"
	"image/png"
	"io"
	"log"
	mangarock "mangapplizer-backend/pkg/parser/mangarock"
	"net/http"
	"os"
	"strings"
)

const (
	anilistURL = "https://graphql.anilist.co/"
)

func mriTest(mr *mangarock.Client) {
	// m, e := mr.Manga("mrs-serie-100177863")
	m, e := mr.Chapter("mrs-serie-100177863", "mrs-chapter-100177864")
	if e != nil {
		log.Fatal(e)
		return
	}
	fmt.Println(m.Pages[0])

	saveMRI(m.Pages[0])

	// testing mri to png
	r, err := os.Open("/tmp/tmp.mri")
	if err != nil {
		log.Fatal("Cannot open the file")
	}
	img, _, err := image.Decode(r)
	if err != nil {
		log.Fatal("Cannot decode the MRI file")
	}

	out := strings.TrimSuffix("/tmp/tmp.mri", ".mri") + ".png"

	w, err := os.Create(out)
	if err != nil {
		log.Fatal("cannot create the outpu")
	}
	if err := png.Encode(w, img); err != nil {
		log.Fatal("Could not encode png")
	}
}

func saveMRI(url string) {
	response, e := http.Get(url)
	if e != nil {
		log.Fatal(e)
	}
	defer response.Body.Close()

	//open a file for writing
	file, err := os.Create("/tmp/tmp.mri")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Use io.Copy to just dump the response body to the file. This supports huge files
	_, err = io.Copy(file, response.Body)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Success!")
}

func main() {
	// q := anilist.New(anilistURL)
	// resp, _ := q.PostPayload()
	// fmt.Println(resp)

	// Testing mangarock
	options := make(map[string]string)
	options["country"] = "United States"
	mr := mangarock.New(mangarock.WithOptions(options))
	mriTest(mr)

}
