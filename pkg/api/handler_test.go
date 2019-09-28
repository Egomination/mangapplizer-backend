package api

import (
	"encoding/json"
	"io/ioutil"
	"net/http"
	"net/http/httptest"
	"testing"
)

const (
	basicAuthUser = "admin"
	basicAuthPass = "test"
)

type response struct {
	Body   string `json:"body"`
	Status string `json:"status"`
}

func TestRootHandler401(t *testing.T) {
	handler := setupRouter(basicAuthUser, basicAuthPass)
	w := httptest.NewRecorder()
	req := httptest.NewRequest("GET", "/api/v1/", nil)
	req.SetBasicAuth("random", "stuff")

	handler.ServeHTTP(w, req)

	resp := w.Result()

	if resp.StatusCode != http.StatusUnauthorized {
		t.Errorf("Unexpected status code %d", resp.StatusCode)
	}
}

func TestRootHandler200(t *testing.T) {
	handler := setupRouter(basicAuthUser, basicAuthPass)
	w := httptest.NewRecorder()
	req := httptest.NewRequest("GET", "/api/v1/", nil)
	req.SetBasicAuth(basicAuthUser, basicAuthPass)

	handler.ServeHTTP(w, req)

	resp := w.Result()

	if resp.StatusCode != http.StatusOK {
		t.Errorf("Unexpected status code %d", resp.StatusCode)
	}

	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)

	if err != nil {
		t.Errorf("Unexpected error! Cannot read the response body!")
	}

	var r response

	err = json.Unmarshal(body, &r)

	if err != nil {
		t.Errorf("Unexpected error! Cannot parse the response body!")
	}

	if r.Body != "Welcome!" || r.Status != "ok" {
		t.Errorf("Unexpected error!")
	}
}

func TestSearchManga(t *testing.T) {
	handler := setupRouter(basicAuthUser, basicAuthPass)
	w := httptest.NewRecorder()
	req := httptest.NewRequest("GET", "/api/v1/search/Test", nil)
	req.SetBasicAuth(basicAuthUser, basicAuthPass)

	handler.ServeHTTP(w, req)

	resp := w.Result()

	if resp.StatusCode != http.StatusOK {
		t.Errorf("Unexpected status code %d", resp.StatusCode)
	}

	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)

	if err != nil {
		t.Errorf("Unexpected error! Cannot read the response body!")
	}

	var r response

	err = json.Unmarshal(body, &r)

	if err != nil {
		t.Errorf("Unexpected error! Cannot parse the response body!")
	}

	if r.Body != "Test" || r.Status != "ok" {
		t.Errorf("Unexpected error!")
	}

}
