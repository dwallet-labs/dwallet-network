# Run a Ika Node using Systemd

Tested using:
- Ubuntu 20.04 (linux/amd64) on bare metal
- Ubuntu 22.04 (linux/amd64) on bare metal

## Prerequisites and Setup

1. Add a `ika` user and the `/opt/ika` directories

```shell
sudo useradd ika
sudo mkdir -p /opt/ika/bin
sudo mkdir -p /opt/ika/config
sudo mkdir -p /opt/ika/db
sudo mkdir -p /opt/ika/key-pairs
sudo chown -R ika:ika /opt/ika
```

2. Install the Ika Node (ika-node) binary, two options:
    
- Pre-built binary stored in Amazon S3:
        
```shell
wget https://releases.ika.io/$IKA_SHA/ika-node
chmod +x ika-node
sudo mv ika-node /opt/ika/bin
```

- Build from source:

```shell
git clone https://github.com/MystenLabs/sui.git && cd ika
git checkout $IKA_SHA
cargo build --release --bin ika-node
mv ./target/release/ika-node /opt/ika/bin/ika-node
```

3. Copy your key-pairs into `/opt/ika/key-pairs/` 

If generated during the Genesis ceremony these will be at `IkaExternal.git/ika-testnet-wave3/genesis/key-pairs/`

Make sure when you copy them they retain `ika` user permissions. To be safe you can re-run: `sudo chown -R ika:ika /opt/ika`

4. Update the node configuration file and place it in the `/opt/ika/config/` directory.

Add the paths to your private keys to validator.yaml. If you chose to put them in `/opt/ika/key-pairs`, you can use the following example: 

```
protocol-key-pair: 
  path: /opt/ika/key-pairs/protocol.key
worker-key-pair: 
  path: /opt/ika/key-pairs/worker.key
network-key-pair: 
  path: /opt/ika/key-pairs/network.key
```

5. Place genesis.blob in `/opt/ika/config/` (should be available after the Genesis ceremony)

6. Copy the ika-node systemd service unit file 

File: [ika-node.service](./ika-node.service)

Copy the file to `/etc/systemd/system/ika-node.service`.

7. Reload systemd with this new service unit file, run:

```shell
sudo systemctl daemon-reload
```

8. Enable the new service with systemd

```shell
sudo systemctl enable ika-node.service
```

## Connectivity

You may need to explicitly open the ports outlined in [Ika for Node Operators](../ika_for_node_operators.md#connectivity) for the required Ika Node connectivity.

## Start the node

Start the Validator:

```shell
sudo systemctl start ika-node
```

Check that the node is up and running:

```shell
sudo systemctl status ika-node
```

Follow the logs with:

```shell
journalctl -u ika-node -f
```

## Updates

When an update is required to the Ika Node software the following procedure can be used. It is highly **unlikely** that you will want to restart with a clean database.

- assumes ika-node lives in `/opt/ika/bin/`
- assumes systemd service is named ika-node
- **DO NOT** delete the Ika databases

1. Stop ika-node systemd service

```
sudo systemctl stop ika-node
```

2. Fetch the new ika-node binary

```shell
wget https://releases.ika.io/${IKA_SHA}/ika-node
```

3. Update and move the new binary:

```
chmod +x ika-node
sudo mv ika-node /opt/ika/bin/
```

4. start ika-node systemd service

```
sudo systemctl start ika-node
```
