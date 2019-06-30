package anilist

import (
	"reflect"
	"testing"
)

func TestPrepeareRequetsBody(t *testing.T) {
	query := "{\"text\": \"Hello, World!\"}"
	res, err := prepeareRequestBody(query)
	if err != nil {
		t.Fatal(err)
	}
	expected := []byte(query)
	if reflect.DeepEqual(res, expected) {
		t.Errorf("Marshalling Error!: got %v want %v",
			res, expected)
	}
}

func TestPrepeareRequest(t *testing.T) {
	url := "http://google.com"

	resp, err := prepeareRequest(url)
	if err != nil {
		t.Fatal(err)
	}
	expectedHeader := "application/json"
	if resp.Header.Get("Content-Type") != expectedHeader {
		t.Errorf("Content type missmatch: got %v want %v",
			resp.Header.Get("Content-Type"), expectedHeader)
	}
	if resp.URL.String() != url {
		t.Errorf("Url missmatch: got %v want %v",
			resp.URL.String(), url)
	}
}

func TestPostPayload(t *testing.T) {
	// I don't know this link will expire or not.
	q := New("https://maap-test.free.beeceptor.com/mock/api/getMedia")
	data, err := q.PostPayload()
	if err != nil {
		t.Fatal(err)
	}
	expectedID := int64(3001)
	if data.Data.Page.Media[0].ID == expectedID {
		t.Errorf("Resp is not what we want: got %v want %v",
			data.Data.Page.Media[0].ID, expectedID)
	}
}
