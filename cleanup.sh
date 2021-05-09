echo "Cleaning up installed pip dependecies"
pip3 uninstall stickytape -y
pip3 uninstall -r src/builtins/prog_sources/iCloudCLI/requirements.txt -y
rm -rf src/builtins/prog_sources