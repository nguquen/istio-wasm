1. Start local k8s with minibute:
```
minikube start --memory 8192
```

2. Install Istio:
```
brew install istioctl
istioctl install --set profile=demo -y
kubectl label namespace default istio-injection=enabled
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
