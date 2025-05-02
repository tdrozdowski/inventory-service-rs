# Kubernetes Manifests with Kustomize

This directory contains Kubernetes manifests for deploying the inventory service, organized using Kustomize.

## Structure

- `base/`: Contains the base manifests without namespace specification
  - `service.yaml`: Service definition
  - `deployment.yaml`: Deployment definition
  - `config-map.yaml`: ConfigMap definition
  - `service-monitor.yaml`: ServiceMonitor definition
  - `kustomization.yaml`: Kustomize configuration for base resources

- `overlays/`: Contains environment-specific overlays
  - `examples/`: Overlay for the "examples" environment
    - `kustomization.yaml`: Kustomize configuration that sets the namespace to "examples"

## Usage

### View the manifests for a specific environment

```bash
cd manifests/overlays/examples
kustomize build
```

### Apply the manifests to a Kubernetes cluster

```bash
cd manifests/overlays/examples
kustomize build | kubectl apply -f -
```

### Adding a new environment

1. Create a new directory under `overlays/` for your environment (e.g., `overlays/production/`)
2. Create a `kustomization.yaml` file in the new directory with the appropriate namespace and any environment-specific patches or configurations

Example:

```yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

namespace: production

resources:
  - ../../base

# Add environment-specific patches if needed
# patches:
#   - path: patch.yaml
```