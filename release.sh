set -ex

# -- configuration --
VERSION=1.0.0
# -- end config --

cargo build --release

rm -rf release
mkdir release

cp target/release/logmove-server.exe release
cp target/release/logmove-client.exe release

cp example-config.yaml release/config.yaml
cp example-client-config.yaml release/client-config.yaml
cp Rocket.toml release

cd release
SERVER_ZIP=logmove-server_$VERSION.zip
CLIENT_ZIP=logmove-client_$VERSION.zip
7z a -tzip $SERVER_ZIP Rocket.toml logmove-server.exe config.yaml
7z a -tzip $CLIENT_ZIP client-config.yaml logmove-client.exe
mkdir zips
mv $SERVER_ZIP $CLIENT_ZIP zips