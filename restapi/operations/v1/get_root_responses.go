// Code generated by go-swagger; DO NOT EDIT.

package v1

// This file was generated by the swagger tool.
// Editing this file might prove futile when you re-run the swagger generate command

import (
	"net/http"

	"github.com/go-openapi/runtime"

	models "mangapplizer-backend/models"
)

// GetRootOKCode is the HTTP code returned for type GetRootOK
const GetRootOKCode int = 200

/*GetRootOK Successfull Response

swagger:response getRootOK
*/
type GetRootOK struct {

	/*
	  In: Body
	*/
	Payload string `json:"body,omitempty"`
}

// NewGetRootOK creates GetRootOK with default headers values
func NewGetRootOK() *GetRootOK {

	return &GetRootOK{}
}

// WithPayload adds the payload to the get root o k response
func (o *GetRootOK) WithPayload(payload string) *GetRootOK {
	o.Payload = payload
	return o
}

// SetPayload sets the payload to the get root o k response
func (o *GetRootOK) SetPayload(payload string) {
	o.Payload = payload
}

// WriteResponse to the client
func (o *GetRootOK) WriteResponse(rw http.ResponseWriter, producer runtime.Producer) {

	rw.WriteHeader(200)
	payload := o.Payload
	if err := producer.Produce(rw, payload); err != nil {
		panic(err) // let the recovery middleware deal with this
	}
}

// GetRootUnauthorizedCode is the HTTP code returned for type GetRootUnauthorized
const GetRootUnauthorizedCode int = 401

/*GetRootUnauthorized Authentication Error

swagger:response getRootUnauthorized
*/
type GetRootUnauthorized struct {

	/*
	  In: Body
	*/
	Payload *models.AuthenticationError `json:"body,omitempty"`
}

// NewGetRootUnauthorized creates GetRootUnauthorized with default headers values
func NewGetRootUnauthorized() *GetRootUnauthorized {

	return &GetRootUnauthorized{}
}

// WithPayload adds the payload to the get root unauthorized response
func (o *GetRootUnauthorized) WithPayload(payload *models.AuthenticationError) *GetRootUnauthorized {
	o.Payload = payload
	return o
}

// SetPayload sets the payload to the get root unauthorized response
func (o *GetRootUnauthorized) SetPayload(payload *models.AuthenticationError) {
	o.Payload = payload
}

// WriteResponse to the client
func (o *GetRootUnauthorized) WriteResponse(rw http.ResponseWriter, producer runtime.Producer) {

	rw.WriteHeader(401)
	if o.Payload != nil {
		payload := o.Payload
		if err := producer.Produce(rw, payload); err != nil {
			panic(err) // let the recovery middleware deal with this
		}
	}
}