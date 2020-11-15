# Deploy application on kubernetes


Make sure you have minikube running. Deploying it on a cloud should only required small change like the `storageClass`

**/!\ This will only deploy the app**

## Requirements

- [Docker](https://www.docker.com/)
- [Kubernetes](https://kubernetes.io/)
- [Helm](https://helm.sh)

## Usage

```sh
# export expected variable
export DB_USER=rustipe
export DB_PASSWORD=some-password
export DB_HOST=some-host.com
export JWT_SECRET=some-secret # if you let this, an attacker will be able to create valid JWT token
export AWS_ACCESS_KEY_ID=some-id
export AWS_SECRET_ACCESS_KEY=some-secret

# install the pod application pod
helm install rustipe-dev ./rustipe --set db.user=${DB_USER} --set db.password=${DB_PASSWORD} --set db.host=${DB_HOST} --set jwt.secret=${JWT_SECRET} --set aws.access_key_id=${AWS_ACCESS_KEY_ID} --set aws.secret_access_key=${AWS_SECRET_ACCESS_KEY}
```

## Todo

- Run the migration ?
- Run postgres under a feature flag.
