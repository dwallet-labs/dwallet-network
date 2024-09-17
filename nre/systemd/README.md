# Run a Pera Node using Systemd

Tested using:
- Ubuntu 20.04 (linux/amd64) on bare metal
- Ubuntu 22.04 (linux/amd64) on bare metal

## Prerequisites and Setup

1. Add a `pera` user and the `/opt/pera` directories

```shell
sudo useradd pera
sudo mkdir -p /opt/pera/bin
sudo mkdir -p /opt/pera/config
sudo mkdir -p /opt/pera/db
sudo mkdir -p /opt/pera/key-pairs
sudo chown -R pera:pera /opt/pera
```

2. Install the Pera Node (pera-node) binary, two options:
    
- Pre-built binary stored in Amazon S3:
        
```shell
wget https://releases.pera.io/$PERA_SHA/pera-node
chmod +x pera-node
sudo mv pera-node /opt/pera/bin
```

- Build from source:

```shell
git clone https://github.com/MystenLabs/sui.git && cd pera
git checkout $PERA_SHA
cargo build --release --bin pera-node
mv ./target/release/pera-node /opt/pera/bin/pera-node
```

3. Copy your key-pairs into `/opt/pera/key-pairs/` 

If generated during the Genesis ceremony these will be at `PeraExternal.git/pera-testnet-wave3/genesis/key-pairs/`

Make sure when you copy them they retain `pera` user permissions. To be safe you can re-run: `sudo chown -R pera:pera /opt/pera`

4. Update the node configuration file and place it in the `/opt/pera/config/` directory.

Add the paths to your private keys to validator.yaml. If you chose to put them in `/opt/pera/key-pairs`, you can use the following example: 

```
protocol-key-pair: 
  path: /opt/pera/key-pairs/protocol.key
worker-key-pair: 
  path: /opt/pera/key-pairs/worker.key
network-key-pair: 
  path: /opt/pera/key-pairs/network.key
```

5. Place genesis.blob in `/opt/pera/config/` (should be available after the Genesis ceremony)

6. Copy the pera-node systemd service unit file 

File: [pera-node.service](./pera-node.service)

Copy the file to `/etc/systemd/system/pera-node.service`.

7. Reload systemd with this new service unit file, run:

```shell
sudo systemctl daemon-reload
```

8. Enable the new service with systemd

```shell
sudo systemctl enable pera-node.service
```

## Connectivity

You may need to explicitly open the ports outlined in [Pera for Node Operators](../pera_for_node_operators.md#connectivity) for the required Pera Node connectivity.

## Start the node

Start the Validator:

```shell
sudo systemctl start pera-node
```

Check that the node is up and running:

```shell
sudo systemctl status pera-node
```

Follow the logs with:

```shell
journalctl -u pera-node -f
```

## Updates

When an update is required to the Pera Node software the following procedure can be used. It is highly **unlikely** that you will want to restart with a clean database.

- assumes pera-node lives in `/opt/pera/bin/`
- assumes systemd service is named pera-node
- **DO NOT** delete the Pera databases

1. Stop pera-node systemd service

```
sudo systemctl stop pera-node
```

2. Fetch the new pera-node binary

```shell
wget https://releases.pera.io/${PERA_SHA}/pera-node
```

3. Update and move the new binary:

```
chmod +x pera-node
sudo mv pera-node /opt/pera/bin/
```

4. start pera-node systemd service

```
sudo systemctl start pera-node
```
