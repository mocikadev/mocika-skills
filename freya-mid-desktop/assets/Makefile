APP_NAME ?= $(shell awk -F '\"' '/^name =/ { print $$2; exit }' Cargo.toml)
VERSION ?= $(shell awk -F '\"' '/^version =/ { print $$2; exit }' Cargo.toml)
DIST_DIR := dist

.PHONY: all clean setup-linux package-linux package-windows package-macos-intel package-macos-arm

all: package-linux package-windows package-macos-intel package-macos-arm

clean:
	cargo clean
	rm -rf $(DIST_DIR)

$(DIST_DIR):
	mkdir -p $(DIST_DIR)

setup-linux:
	sudo apt-get update
	sudo apt-get install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# 使用 cargo-packager 生成标准的桌面系统安装包，并强行规范化命名规则为：AppName_Version_OS_Arch.ext
package-linux: $(DIST_DIR)
	cargo packager --release --formats appimage,deb
	@cd $(DIST_DIR) && for file in *; do \
		if [ -f "$$file" ]; then \
			ext="$${file##*.}"; \
			mv "$$file" "$(APP_NAME)_$(VERSION)_linux_amd64.$$ext"; \
		fi \
	done

package-windows: $(DIST_DIR)
	cargo packager --release --formats msi,nsis
	@cd $(DIST_DIR) && for file in *; do \
		if [ -f "$$file" ]; then \
			ext="$${file##*.}"; \
			mv "$$file" "$(APP_NAME)_$(VERSION)_windows_x64.$$ext"; \
		fi \
	done

package-macos-intel: $(DIST_DIR)
	cargo packager --release --target x86_64-apple-darwin --formats dmg
	@cd $(DIST_DIR) && for file in *; do \
		if [ -f "$$file" ]; then \
			ext="$${file##*.}"; \
			mv "$$file" "$(APP_NAME)_$(VERSION)_macos_x64.$$ext"; \
		fi \
	done

package-macos-arm: $(DIST_DIR)
	cargo packager --release --target aarch64-apple-darwin --formats dmg
	@cd $(DIST_DIR) && for file in *; do \
		if [ -f "$$file" ]; then \
			ext="$${file##*.}"; \
			mv "$$file" "$(APP_NAME)_$(VERSION)_macos_arm64.$$ext"; \
		fi \
	done
