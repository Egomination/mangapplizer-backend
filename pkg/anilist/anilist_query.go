package anilist

import (
	"bytes"
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
	"time"
)

const (
	anilistURL = "https://graphql.anilist.co/"
)

// Marshal the query
func prepeareRequestBody() ([]byte, error) {
	// Creating the request body.
	return json.Marshal(map[string]string{
		"query": query,
	})
}

func prepeareRequest() (*http.Request, error) {
	requestBody, err := prepeareRequestBody()
	if err != nil {
		log.Fatal(err)
		panic(nil)
	}
	req, err := http.NewRequest("POST", anilistURL, bytes.NewBuffer(requestBody))
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
func PostPayload() (*Anilist, error) {
	req, err := prepeareRequest()
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
