BUILD_DIR = build
EXEC = $(BUILD_DIR)/PROJ4.exe
VCPKG_BIN = C:/dev/GitHub/CPP/Politechnika/GrafikaKomputerowa/vcpkg/installed/x64-mingw-dynamic/bin
SHADERS_SRC = shaders
SHADERS_DST = $(BUILD_DIR)/shaders

all: clean build copy_shaders

build:
	@echo "Tworzenie katalogu build..."
	@mkdir -p $(BUILD_DIR)
	@cd $(BUILD_DIR) && cmake .. -G "MinGW Makefiles"
	@cd $(BUILD_DIR) && mingw32-make

copy_shaders:
	@echo "Kopiowanie glfw3.dll..."
	@cp $(VCPKG_BIN)/glfw3.dll $(BUILD_DIR)/
	@echo "Kopiowanie shaderów..."
	@mkdir -p $(SHADERS_DST)
	@cp -r $(SHADERS_SRC)/* $(SHADERS_DST)/
	@echo "Kopiowanie tekstur..."
	@cp wall.jpg $(BUILD_DIR)/

clean:
	@echo "Czyszczenie..."
	@rm -rf $(BUILD_DIR)
