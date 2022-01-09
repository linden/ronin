# Shell script to initialize a Ubuntu 20.04 LTS server with Hiro's API

export MAINNET_ARCHIVE=https://docker.06815d71-a2bc-4176-98bb-dccd6c237f84.uk-lon1.upcloudobjects.com/mainnet.tar.gz

echo "Updating APT..."

apt-get update -qq

if [ -x "$(command -v docker)" ]; then
    echo "Docker is installed, skipping install..."
else
    echo "Installing Docker..."
    curl -fsSL https://get.docker.com -o get-docker.sh
    sh get-docker.sh
fi

if [ -x "$(command -v cargo)" ]; then
    echo "Cargo is installed, skipping install..."
else
    echo "Installing Cargo..."
    apt install -y cargo -qq
fi

echo "Installing JQ, PostgreSQL client & PV..."

apt-get install postgresql-client-common jq pv -y -qq

echo "Installing b3sum..."
cargo install b3sum -q
export PATH=/root/.cargo/bin:$PATH

VERSION=$(curl --silent https://api.github.com/repos/docker/compose/releases/latest | jq .name -r)
echo "Installing Docker Compose $VERSION"
DESTINATION=/usr/local/bin/docker-compose
sudo curl -L --silent https://github.com/docker/compose/releases/download/${VERSION}/docker-compose-$(uname -s)-$(uname -m) -o $DESTINATION
sudo chmod 755 $DESTINATION

echo "Checking for existing archive..."
if [ -f "mainnet.tar.gz" ]; then
    echo "Archive found, skipping download..."
else
    echo "Downloading node + snapshot..."
    curl $MAINNET_ARCHIVE -o mainnet.tar.gz
fi

echo "Verifiying integrity..."

export EXPECTED=56116b8c0175cf0f2d5acd9e1664b2edcde9bf256628524d99cd34d0b5c8e1c8

if pv mainnet.tar.gz | b3sum --no-names | grep -q $EXPECTED; then 
    echo "Integrity at 100%."
else
    echo "Integrity at 0%. Removing archive. Please try again."
    rm mainnet.tar.gz
    exit 1
fi

echo "Extracting node..."

pv mainnet.tar.gz | tar -xz

cd stacks-local-dev

cp sample.env .env

echo "Booting node..."

./manage.sh mainnet up