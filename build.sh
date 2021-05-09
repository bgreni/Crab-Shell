
ROOT=`pwd`
PROG_SOURCE_ROOT="${ROOT}/src/builtins/prog_sources"
BIN="${ROOT}/src/builtins/bin"
INSTALL_FAIL="Failed to install builtin: "

[ ! -d "${PROG_SOURCE_ROOT}" ] && mkdir "${PROG_SOURCE_ROOT}"


if ! command -v git &> /dev/null
then
  echo "Please install git to complete installation"
  exit 1
fi

if ! command -v pip3 &> /dev/null
then
  echo "Please install pip3 to complete installation"
  exit 1
fi

cd "${PROG_SOURCE_ROOT}" || exit 1

if [ ! -d "QSFactCpp" ]
then
  echo "Cloning and Building QSFactCpp"
  git clone --quiet https://github.com/bgreni/QSFactCpp.git > /dev/null
  (cd QSFactCpp && ./build.sh > /dev/null && mv src/qsmain "${BIN}/qsmain") || (echo "${INSTALL_FAIL} QSFactCpp" && rm -r QSFactCpp && exit 1)
fi

if [ ! -d "iCloudCLI" ]
then
  echo "Cloning and Building iCloudCLI"
  git clone --quiet https://github.com/bgreni/iCloudCLI.git > /dev/null
  cd iCloudCLI || exit 1
  pip3 -q install stickytape
  pip3 -q install -r requirements.txt
  stickytape src/main.py --add-python-path src/ --copy-shebang --output-file "${ROOT}/src/builtins/bin/icloudCLI"
  chmod u+x "${ROOT}/src/builtins/bin/icloudCLI"
  cd ..
fi

[ -f "crab-shell" ] && rm crab-shell

cd "${ROOT}" || exit 1
echo "Building Crab-Shell"
cargo build --release
cp target/release/crab-shell crab-shell