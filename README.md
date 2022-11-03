# unseald
A simple program that checks a HashiCorp Vault status and supplies a key portion if sealed.

## Usage
On Docker:
```console
docker run -e KEY=1234 -e VAULT_ADDR=http://localhost:8200 nnari/unseald
```

## Config options
unseald looks at the following environment variables:
- KEY - The key to unseal with
- VAULT_ADDR - Vault address
- INTERVAL - Defaults to 5, how often to check `/v1/sys/health`
