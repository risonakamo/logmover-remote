set -ex

# -- configuration --
VERSION=1.0.0
# -- end config --

cargo build --release

rm -rf release
mkdir release

cp target/release/logmove-server.exe release
cp target/release/logmove-client.exe release

cp example-config/example-config.yaml release/config.yaml
cp example-config/example-client-config.yaml release/client-config.yaml
cp Rocket.toml release
cp run-scripts/* release

cd release
SERVER_ZIP=logmove-server_$VERSION.zip
CLIENT_ZIP=logmove-client_$VERSION.zip
7z a -tzip $SERVER_ZIP Rocket.toml logmove-server.exe config.yaml logmove-server.bat
7z a -tzip $CLIENT_ZIP client-config.yaml logmove-client.exe logmove-client.bat
mkdir zips
mv $SERVER_ZIP $CLIENT_ZIP zips