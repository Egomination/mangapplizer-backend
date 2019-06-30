package anilist

import (
	"bytes"
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
	"time"
)

// Query is the simplest query struct for passing url as a param instead of
// having constant
type Query struct {
	URL string
}

// New query instance
func New(url string) *Query {
	return &Query{
		URL: url,
	}
}

// Marshal the query
func prepeareRequestBody(query string) ([]byte, error) {
	// Creating the request body.
	return json.Marshal(map[string]string{
		"query": query,
	})
}

func prepeareRequest(url string) (*http.Request, error) {
	requestBody, err := prepeareRequestBody(query)
	if err != nil {
		log.Fatal(err)
		panic(nil)
	}
	req, err := http.NewRequest("POST", url, bytes.NewBuffer(requestBody))
	req.Header.Set("Content-Type", "application/json")

	return req, err
}

// PostPayload sends the scan query and returns to the Anilist struct
//
// Usage:
// `
// mediaData, err := PostPayload()
// if err != nill {
//	panic(err)
//}
// type:=mediaData.Data.Page.Media[0].Relations.Edges[0].Node.Type
// fmt.Println(type)
// `
//
func (q *Query) PostPayload() (*Anilist, error) {
	req, err := prepeareRequest(q.URL)
	if err != nil {
		panic(err)
	}
	timeout := time.Duration(10 * time.Second)
	client := &http.Client{
		Timeout: timeout,
	}
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()
	return returnMediaData(resp)
}

func returnMediaData(resp *http.Response) (*Anilist, error) {
	body, _ := ioutil.ReadAll(resp.Body)
	// string(body)
	mediaData, err := UnmarshalAnilist(body)
	return &mediaData, err
}
