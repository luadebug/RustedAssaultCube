# Internal DLL cheat, you need to build x86 and inject DLL into game process Assault Cube 1.3.0.2
# Triggerbot, Aimbot, ESP and utils for Assault Cube 1.3.0.2
# Game's backend is OPENGL
# DLL uses SimHei.ttf to render chinese glyphs, make sure you have installed SimHei.ttf into your Windows/Fonts folder.
# SimHei.ttf download link: https://huggingface.co/internlm/internlm-xcomposer2d5-7b/resolve/main/SimHei.ttf
# DLL uses HUDHOOK (IMGUI for menu and for drawing objects for extrasensory perception)
# This DLL does not use OTF fonts yet, but OTF fonts support is enabled, to use it you would need to install freetype using vcpkg
# To install freetype you need to install vcpkg and install freetype, using command 
# vcpkg install freetype --triplet=x86-windows-static-md && vcpkg install freetype --triplet=x86-windows-static && vcpkg install freetype --triplet=x86-windows
# later once you installed packages with vcpkg make sure you made vcpkg integrated.
# vcpkg integrate install
# Note it uses nightly rust version and game is supported only for x86
# To build you need to make sure you installed freetype using vcpkg and run command line cargo build -vv --target=i686-pc-windows-msvc --release
![Preview](https://raw.githubusercontent.com/luadebug/RustedAssaultCube/main/Preview.png)
![Preview2](https://raw.githubusercontent.com/luadebug/RustedAssaultCube/main/Preview2.png)
![Preview3](https://raw.githubusercontent.com/luadebug/RustedAssaultCube/main/Preview3.png)
