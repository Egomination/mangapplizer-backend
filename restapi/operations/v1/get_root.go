// Code generated by go-swagger; DO NOT EDIT.

package v1

// This file was generated by the swagger tool.
// Editing this file might prove futile when you re-run the generate command

import (
	"net/http"

	middleware "github.com/go-openapi/runtime/middleware"

	models "mangapplizer-backend/models"
)

// GetRootHandlerFunc turns a function with the right signature into a get root handler
type GetRootHandlerFunc func(GetRootParams, *models.Principal) middleware.Responder

// Handle executing the request and returning a response
func (fn GetRootHandlerFunc) Handle(params GetRootParams, principal *models.Principal) middleware.Responder {
	return fn(params, principal)
}

// GetRootHandler interface for that can handle valid get root params
type GetRootHandler interface {
	Handle(GetRootParams, *models.Principal) middleware.Responder
}

// NewGetRoot creates a new http.Handler for the get root operation
func NewGetRoot(ctx *middleware.Context, handler GetRootHandler) *GetRoot {
	return &GetRoot{Context: ctx, Handler: handler}
}

/*GetRoot swagger:route GET /v1 v1 getRoot

Get root of the API V1

"Just printing hello world message to the user"


*/
type GetRoot struct {
	Context *middleware.Context
	Handler GetRootHandler
}

func (o *GetRoot) ServeHTTP(rw http.ResponseWriter, r *http.Request) {
	route, rCtx, _ := o.Context.RouteInfo(r)
	if rCtx != nil {
		r = rCtx
	}
	var Params = NewGetRootParams()

	uprinc, aCtx, err := o.Context.Authorize(r, route)
	if err != nil {
		o.Context.Respond(rw, r, route.Produces, route, err)
		return
	}
	if aCtx != nil {
		r = aCtx
	}
	var principal *models.Principal
	if uprinc != nil {
		principal = uprinc.(*models.Principal) // this is really a models.Principal, I promise
	}

	if err := o.Context.BindValidRequest(r, route, &Params); err != nil { // bind params
		o.Context.Respond(rw, r, route.Produces, route, err)
		return
	}

	res := o.Handler.Handle(Params, principal) // actually handle the request

	o.Context.Respond(rw, r, route.Produces, route, res)

}