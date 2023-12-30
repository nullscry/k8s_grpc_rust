# Showcasing gRPC, Rust and Kubernetes
This project is for demonstration purposes.

The project uses `tonic` for gRPC APIs, `minikube` for a local Kubernetes cluster, and Postgresql with `diesel` for the database.

## Building the App
Since there are three cargo packages I provided a simple script to build Docker images for each sequentially.
### Build images
```sh
./build_images.sh
```

### Start Minikube
Start your Minikube cluster!
```sh
minikube start
```

### Push locally built images to Minikube
Since built images are stored in whatever container runtime you are using, you need to push them to minikube to render them accessible.

```sh
minikube cache add clock_image:latest
minikube cache add numbers_image:latest
minikube cache add dbapi_image:latest
```

### Create an SSL Certificate and Key
This step is needed since you need to terminate TLS at ingress-nginx level before routing to services. Although I provided a key and certificate for this occassion. I wouldn't do this in a real project of course...

```sh
openssl req -x509 -sha256 -nodes -days 365 -newkey rsa:2048 -keyout tls.key -out tls.crt -subj "/CN=smartmodels.local/O=smartmodels.local" -addext "subjectAltName = DNS:smartmodels.local"
```

### Push SSL Certificate and Key to Minikube Secrets as a TLS secret
We send the certificate and the key, and configure the ingress so it can access it for secure connections.
```sh
kubectl create secret tls mkcert --key tls.key --cert tls.crt
minikube addons configure ingress
minikube addons disable ingress
minikube addons enable ingress
```

### Deploy the Minikube cluster declaratively
```sh
kubectl apply -f k8s_config/workload.yaml -f k8s_config/services.yaml -f k8s_config/ingress.yaml
```

### Tunnel ingress to localhost if needed on a seperate shell
This is necessary for some working environments. I had to since I work on WSL2.
```sh
minikube tunnel
```

### Add ingress hostname to /etc/hosts if you had to tunnel
Since you cannot use IP addresses for SSL, I just added my made up hostname to /etc/hosts, and routed it to 127.0.0.1 since I used tunnel.
```sh
echo "127.0.0.1       smartmodels.local" >> /etc/hosts
```

### Open up the dashboard in another terminal if you'd like
For easier control over your Minikube cluster. And monitoring what's happening inside.
```sh
minikube addons enable dashboard
minikube dashboard
```

## Testing the App
I used `grpcurl` for all my testings. I chose to use this tool instead of a `tonic` client since I wanted to keep it as simple and universal as possible. I provided the binary for `grpcurl` for convenience.

### Clock SmartModel
```sh
./grpcurl -insecure -import-path ./proto -proto clock.proto -d '{"name": "Midge Ure"}' smartmodels.local:443 clock.Clock/SayHello
./grpcurl -insecure -import-path ./proto -proto clock.proto -d '{"name": "Robert Smith"}' smartmodels.local:443 clock.Clock/SayHelloAgain
```

### Numbers SmartModel
```sh
./grpcurl -insecure -import-path ./proto -proto numbers.proto -d '{"lhs": 5, "rhs": 25}' smartmodels.local:443 numbers.Calculate/AddNumbers
./grpcurl -insecure -import-path ./proto -proto numbers.proto -d '{"lhs": 5, "rhs": 25}' smartmodels.local:443 numbers.Calculate/MultNumbers
```

### The DB gRPC api
```sh
./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"name": "clock_tonic", "category": "wearable"}' smartmodels.local:443 dbapi.Database/InsertModel
./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"name": "numbers", "category": "wearable"}' smartmodels.local:443 dbapi.Database/InsertModel

./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"name": "clock_tonic"}' smartmodels.local:443 dbapi.Database/SelectModel
./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"category": "wearable"}' smartmodels.local:443 dbapi.Database/SelectModel
./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"name": "clock_tonic"}' smartmodels.local:443 dbapi.Database/SelectModel
./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"name": "numbers", "category": "wearable"}' smartmodels.local:443 dbapi.Database/SelectModel
```

```sh
./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"name": "say_hello", "category": "greeter", "model": "clock_tonic"}' smartmodels.local:443 dbapi.Database/InsertFeature
./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"name": "say_hello_again", "category": "greeter", "model": "clock_tonic"}' smartmodels.local:443 dbapi.Database/InsertFeature
./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"name": "add_numbers", "category": "calculator", "model": "numbers"}' smartmodels.local:443 dbapi.Database/InsertFeature
./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"name": "mult_numbers", "category": "calculator", "model": "numbers"}' smartmodels.local:443 dbapi.Database/InsertFeature

./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"model": "clock_tonic"}' smartmodels.local:443 dbapi.Database/SelectFeature
./grpcurl -insecure -import-path ./proto -proto dbapi.proto -d '{"name": "mult_numbers", "category": "calculator"}' smartmodels.local:443 dbapi.Database/SelectFeature
```