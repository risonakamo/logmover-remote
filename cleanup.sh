# clean up all dev files EXCEPT for test files

set -ex

rm -rf release target

cd remote-rename-web
rm -rf build node_modules release