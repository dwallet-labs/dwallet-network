## Steps to deploy the Ika network with a configurable scale on a Kubernetes cluster of your choice.

### 1. Set chain values
Copy the `.env.template` file to `.env` and set the variables in it with configuration of your choice.

### 2. Build the Docker image
Run the following command from this directory to build the ika-node docker image:
```bash
./build.sh
```

### 3. Create Genesis files
Run the following command from this directory to create the genesis files:
```bash
./create-ika-genesis-mac.sh
```

### 4. Deploy the Ika network
Run the `"deploy the ika network from the current directory to the local kubernetes cluster"` test from the 
`./chaos.test.ts` file.

### 5. Run TS tests against the deployed Ika network
First, run the following command from this directory
```bash
cp ./publisher/ika_config.json ../../../../
```
Now, make sure your