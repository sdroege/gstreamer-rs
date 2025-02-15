set -e

RELEASE=v3.0.0

git clone https://github.com/fraunhoferhhi/vvdec.git
cd vvdec
git checkout $RELEASE
cmake -S . -B build -GNinja \
  -DCMAKE_INSTALL_PREFIX=/usr/local \
  -DBUILD_SHARED_LIBS=ON \
  -DVVDEC_TOPLEVEL_OUTPUT_DIRS=OFF
ninja -C build
ninja -C build install
cd ..
rm -rf vvdec
