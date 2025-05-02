# Working with Kubernetes Secrets and Sealed Secrets

This document provides instructions on how to extract Kubernetes secrets from a cluster, convert them to sealed secrets using kubeseal, and manage them in a GitOps workflow.

## Prerequisites

Before you can work with sealed secrets, you need to install the following tools:

### kubectl

kubectl is the Kubernetes command-line tool that allows you to run commands against Kubernetes clusters.

#### macOS
```bash
# Using Homebrew
brew install kubectl

# Or using curl
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl"
chmod +x ./kubectl
sudo mv ./kubectl /usr/local/bin/kubectl
```

#### Linux
```bash
# Using apt (Debian/Ubuntu)
sudo apt-get update && sudo apt-get install -y kubectl

# Using snap
sudo snap install kubectl --classic

# Or using curl
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
chmod +x ./kubectl
sudo mv ./kubectl /usr/local/bin/kubectl
```

#### Windows
```powershell
# Using Chocolatey
choco install kubernetes-cli

# Using scoop
scoop install kubectl

# Or download the binary
curl -LO "https://dl.k8s.io/release/v1.26.0/bin/windows/amd64/kubectl.exe"
# Add the binary to your PATH
```

### kubeseal

kubeseal is a tool for encrypting Kubernetes secrets into sealed secrets.

#### macOS
```bash
# Using Homebrew
brew install kubeseal

# Or using binary release
KUBESEAL_VERSION=$(curl -s https://api.github.com/repos/bitnami-labs/sealed-secrets/releases/latest | jq -r '.tag_name')
curl -Lo kubeseal "https://github.com/bitnami-labs/sealed-secrets/releases/download/${KUBESEAL_VERSION}/kubeseal-darwin-amd64"
chmod +x kubeseal
sudo mv kubeseal /usr/local/bin/
```

#### Linux
```bash
# Using binary release
KUBESEAL_VERSION=$(curl -s https://api.github.com/repos/bitnami-labs/sealed-secrets/releases/latest | jq -r '.tag_name')
curl -Lo kubeseal "https://github.com/bitnami-labs/sealed-secrets/releases/download/${KUBESEAL_VERSION}/kubeseal-linux-amd64"
chmod +x kubeseal
sudo mv kubeseal /usr/local/bin/
```

#### Windows
```powershell
# Using Chocolatey
choco install kubeseal

# Or download the binary
$KUBESEAL_VERSION = (Invoke-RestMethod -Uri "https://api.github.com/repos/bitnami-labs/sealed-secrets/releases/latest").tag_name
Invoke-WebRequest -Uri "https://github.com/bitnami-labs/sealed-secrets/releases/download/${KUBESEAL_VERSION}/kubeseal-windows-amd64.exe" -OutFile "kubeseal.exe"
# Add the binary to your PATH
```

## Extracting Secrets from a Kubernetes Cluster

To extract a secret from a Kubernetes cluster, use the following command:

```bash
kubectl get secret <secret-name> -n <namespace> -o yaml > <secret-name>-extracted-secret.yaml
```

For example, to extract the `sqlx-db-secret` from the `examples` namespace:

```bash
kubectl get secret sqlx-db-secret -n examples -o yaml > sqlx-db-extracted-secret.yaml
```

## Cleaning the Extracted Secret

The extracted secret will contain some metadata that you might want to remove before sealing it. You can clean the secret using the following steps:

1. Remove the `creationTimestamp`, `resourceVersion`, `uid`, and other cluster-specific metadata
2. Remove the `ownerReferences` section if present
3. Keep the `name`, `namespace`, and `labels` as needed

## Converting Secrets to Sealed Secrets

To convert an extracted secret to a sealed secret, use the following command:

```bash
kubeseal --format yaml < <secret-name>-extracted-secret.yaml > <secret-name>-sealed-secret.yaml
```

For example, to convert the `sqlx-db-extracted-secret.yaml` to a sealed secret:

```bash
kubeseal --format yaml < sqlx-db-extracted-secret.yaml > sqlx-db-sealed-secret.yaml
```

## Example: Complete Workflow

Here's a complete example of extracting a secret, cleaning it, and converting it to a sealed secret:

```bash
# Extract the secret
kubectl get secret sqlx-db-secret -n examples -o yaml > sqlx-db-extracted-secret.yaml

# Clean the secret (optional)
# You can use tools like yq or manually edit the file

# Convert to sealed secret
kubeseal --format yaml < sqlx-db-extracted-secret.yaml > sqlx-db-sealed-secret.yaml

# Verify the sealed secret
cat sqlx-db-sealed-secret.yaml

# Add the sealed secret to your repository
git add sqlx-db-sealed-secret.yaml
```

## Managing Multiple Secrets

If you need to extract and seal multiple secrets, you can use a script like this:

```bash
#!/bin/bash
NAMESPACE="examples"
SECRETS=("sqlx-db-secret" "inventory-service-jwt-secret" "ghcr-login-secret")

for SECRET in "${SECRETS[@]}"; do
  echo "Extracting $SECRET..."
  kubectl get secret $SECRET -n $NAMESPACE -o yaml > "$SECRET-extracted-secret.yaml"
  
  echo "Sealing $SECRET..."
  kubeseal --format yaml < "$SECRET-extracted-secret.yaml" > "$SECRET-sealed-secret.yaml"
  
  echo "$SECRET sealed successfully!"
done

echo "All secrets have been extracted and sealed!"
```

## Security Considerations

- The extracted secret files contain sensitive information and should not be committed to version control
- The `.gitignore` file in this repository is configured to ignore files with the pattern `*-extracted-secret.yaml`
- Only the sealed secrets should be committed to the repository
- Make sure to delete the extracted secret files after you've created the sealed secrets

## Adding Sealed Secrets to Kustomization

After creating the sealed secrets, add them to your Kustomization file:

```yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

namespace: examples

resources:
  - ../../base
  - secrets/sqlx-db-sealed-secret.yaml
  - secrets/inventory-service-jwt-sealed-secret.yaml
  - secrets/ghcr-login-sealed-secret.yaml
```

## Troubleshooting

If you encounter issues with kubeseal, here are some common troubleshooting steps:

1. Ensure the sealed-secrets controller is installed in your cluster
2. Check that you have the correct permissions to access the secrets
3. Verify that the kubeseal version is compatible with your cluster's sealed-secrets controller
4. If you're using a custom certificate, make sure it's properly configured

For more information, refer to the [sealed-secrets documentation](https://github.com/bitnami-labs/sealed-secrets).