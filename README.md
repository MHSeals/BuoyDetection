# BuoyDetection
Buoy detection in Rust  
Designed for Intel Realsense D435 depth camera  
Buoys found using size of color patches on rgb, and getting depth, and using inverse square law  
Uses the Rust realsense libraries (`realsense-rust`, `realsense-sys`)  
Communicates with Python scripts that control movement through port 8080

# Build
To build, you need to install librealsense  
Can be ~~incredibly~~ somewhat inconsistent if you don't build for release  

For Arch:
```console
# Install librealsense
yay -S librealsense
# Clone repo
git clone https://github.com/MHSeals/BuoyDetection
# Build the package for release
cargo build --release
```
