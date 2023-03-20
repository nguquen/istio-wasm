1. Start local k8s with minibute:
```
minikube start --memory 8192
```

2. Install Istio:
```
brew install istioctl
istioctl install --set profile=demo -y
kubectl label namespace default istio-injection=enabled
kubectl apply -f gateway.yaml
```

3. Build authz-filter:

```
cd authz-filter
cargo build --target wasm32-wasi --release
```

4. Build & deploy auth-grpc service:
```
eval $(minikube docker-env)
cd auth-grpc
docker build -t auth-grpc -f Dockerfile .
kubectl apply -f deployment.yaml
```

5. Build & deploy hello service:
```
eval $(minikube docker-env)
cd hello
cp ../authz-filter/target/wasm32-wasi/release/authz_filter.wasm .
docker build -t hello -f Dockerfile .
kubectl apply -f deployment.yaml
kubectl apply -f envoy-filter.yaml
```

6. Build & deploy world service:
```
eval $(minikube docker-env)
cd world
docker build -t world -f Dockerfile .
kubectl apply -f deployment.yaml
```

7. Testing
```
minikube tunnel
curl http://localhost/world
curl http://localhost/hello
curl -H "token: secured" http://localhost/hello
```
