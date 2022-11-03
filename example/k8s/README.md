# Example
This example goes through an example Kubernetes deployment of unseald.

# Base64 the unseal key
If you like yq:
```
yq -i e ".data.key |= \"$(echo -n 'your-unseal-key' | base64)\"" secrets.yml
```
Otherwise, manually:
```
echo -n 'your-unseal-key' | base64
```
and then update `secret.yml` `key` field with the new secret.

Your `secret.yml` should now look like this:
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: unseald-secret
type: Opaque
data:
  key: eW91ci11bnNlYWwta2V5 # your-unseal-key
```

## Configuring the ConfigMap
Substitute `vault_addr` and `interval` for the wanted values.
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: unseald-configmap
data:
  vault_addr: "http://vault-application.vault:8200"
  interval: "5"
```

## Deploy
```
kubectl apply -f .
```