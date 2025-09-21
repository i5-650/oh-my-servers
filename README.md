# Oh My Servers (WIP)

Allow you to dynamically generate alias for your servers.

## Configuration

You have to create a configuration file in `~/.oh-my-servers/config.yaml`.

```yaml
actives:
  - name: ragnar
    ip: 127.0.0.1
    keyPath: ~/servers/ragnar.key
    os: rocky
    user: rocky
  - name: olaf
    ip: 127.0.0.2
    keyPath: ~/servers/olaf.key
    os: rocky
    user: rocky
  - name: sigrid
    ip: 127.0.0.3
    keyPath: ~/servers/sigrid.key
    os: rocky
    user: rocky
inactives:
  - name: thor
    ip: 127.0.0.4
    keyPath: ~/servers/thor.key
    os: rocky
    user: rocky
```

## Usage

To auto create aliases for your servers, add the following line to your `~/.bashrc` or `~/.zshrc` file:

```bash
eval "$(oh-my-servers shell)"
```
