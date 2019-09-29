Again and again. This is the 7th time we're starting this project from scratch. It's the backend service that fetches, publishes, updates the "media"

# How to run
For running this repository locally you've to follow these steps. (After docker, this thing won't be a problem but...)

```bash
go run cmd/mangapplizer-server/main.go --host 0.0.0.0 --port 9092 --tls-certificate testcert.pem --tls-key testssl.pem --tls-port 9093

# NOTE: certs are self generated just for testing the https. Consider using http endpoint for testing.
# Also, if you have swagger binary `https://goswagger.io/install.html` you can serve swagger ui to check documantation.
swagger serve swagger.yml
```
