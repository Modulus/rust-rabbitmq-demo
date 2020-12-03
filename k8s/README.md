# Prerequisites
1. Install the rabbitmq operator

```
kubectl apply -f "https://github.com/rabbitmq/cluster-operator/releases/latest/download/cluster-operator.yml"
```

2. Deploy rabbitmq cluster
```
kubectl apply -f k8s/cluster.yaml
```

3. Deploy monitoring
```
kubectl apply -f k8s/monitoring.yaml
```

4. Extract username and password for new rabbitmq cluster
```
kubectl get secret -n rabbit-demo test-cluster-default-user -o jsonpath="{.data.username}" | base64 --decode
kubectl get secret -n rabbit-demo test-cluster-default-user -o jsonpath="{.data.password}" | base64 --decode
```

5. Install kube-prometheus and the rabbitmq dashboard(s)

Read more here: https://github.com/prometheus-operator/kube-prometheus

6. Deploy apps
```
kubectl apply -f k8s/app.yaml
```

# RabbitMQ Operator

https://www.rabbitmq.com/kubernetes/operator/install-operator.html

kubectl apply -f "https://github.com/rabbitmq/cluster-operator/releases/latest/download/cluster-operator.yml"

kubectl create ns rabbit-demo

kubectl get secret -n rabbit-demo test-cluster-default-user -o jsonpath="{.data.username}" | base64 --decode
kubectl get secret -n rabbit-demo test-cluster-default-user -o jsonpath="{.data.password}" | base64 --decode

# RabbitMQ metrics
https://www.rabbitmq.com/prometheus.html#metric-aggregation


# To get usernames

kubectl get secret -n rabbit-demo test-cluster-default-user -o jsonpath="{.data.username}" | base64 --decode
kubectl get secret -n rabbit-demo test-cluster-default-user -o jsonpath="{.data.password}" | base64 --decode