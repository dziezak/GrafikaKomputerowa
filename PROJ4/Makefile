# Makefile dla projektu PROJ4

# Ścieżki
BUILD_DIR = build
EXEC = $(BUILD_DIR)/PROJ4.exe
VCPKG_BIN = C:/dev/GitHub/CPP/Politechnika/GrafikaKomputerowa/vcpkg/installed/x64-mingw-dynamic/bin
SHADERS_SRC = shaders
SHADERS_DST = $(BUILD_DIR)/shaders

# Domyślne zadanie
all: $(EXEC) copy_shaders run

# Budowanie projektu
$(EXEC):
	@echo "Tworzenie katalogu build..."
	@if not exist $(BUILD_DIR) mkdir $(BUILD_DIR)
	@cd $(BUILD_DIR) && cmake .. -G "MinGW Makefiles"
	@cd $(BUILD_DIR) && mingw32-make

# Kopiowanie plików DLL i shaderów
copy_shaders:
	@echo "Kopiowanie glfw3.dll..."
	@cp $(VCPKG_BIN)/glfw3.dll $(BUILD_DIR)/
	@echo "Kopiowanie katalogu shaders..."
	@mkdir -p $(SHADERS_DST)
	@cp -r $(SHADERS_SRC)/* $(SHADERS_DST)/

# Uruchomienie programu
run:
	@echo "Uruchamianie PROJ4..."
	@cd $(BUILD_DIR) && ./PROJ4.exe

# Czyszczenie
clean:
	@echo "Usuwanie katalogu build..."
	@rm -rf $(BUILD_DIR)

