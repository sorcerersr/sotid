#!/usr/bin/env sh
set -eu

# Builds Sotid from the current working copy and installs it into ~/.local.
# Run this from the root of a Sotid checkout.

main() {
    platform="$(uname -s)"
    if [ "$platform" = "Darwin" ]; then
        platform="macos"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    # Build from the working copy in release mode.
    echo "Building Sotid from source (this may take a while)..."
    cargo build --release --package sotid --package cli

    "$platform"

    if [ "$(command -v sotid)" = "$HOME/.local/bin/sotid" ]; then
        echo "Sotid has been installed. Run with 'sotid'"
    else
        echo "To run Sotid from your terminal, you must add ~/.local/bin to your PATH"
        echo "Run:"

        case "$SHELL" in
            *zsh)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc"
                echo "   source ~/.zshrc"
                ;;
            *fish)
                echo "   fish_add_path -U $HOME/.local/bin"
                ;;
            *)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc"
                echo "   source ~/.bashrc"
                ;;
        esac

        echo "To run Sotid now, '~/.local/bin/sotid'"
    fi
}

linux() {
    target_dir="target/release"

    app_dir="$HOME/.local/sotid.app"
    rm -rf "$app_dir"
    mkdir -p "$app_dir/bin" "$app_dir/libexec" "$app_dir/share/applications" \
        "$app_dir/share/icons/hicolor/512x512/apps" "$app_dir/share/icons/hicolor/1024x1024/apps"

    # Assemble the app directory.
    cp "$target_dir/sotid" "$app_dir/libexec/sotid-editor"
    cp "$target_dir/cli" "$app_dir/bin/sotid"

    # Icons
    cp "crates/sotid/resources/app-icon.png" "$app_dir/share/icons/hicolor/512x512/apps/sotid.png"
    cp "crates/sotid/resources/app-icon@2x.png" "$app_dir/share/icons/hicolor/1024x1024/apps/sotid.png"

    # .desktop file
    export DO_STARTUP_NOTIFY="true"
    export APP_CLI="sotid"
    export APP_ICON="sotid"
    export APP_ARGS="%U"
    export APP_NAME="Sotid"
    mkdir -p "$HOME/.local/share/applications"
    envsubst < "crates/sotid/resources/sotid.desktop.in" > "$app_dir/share/applications/de.sorcarion.Sotid.desktop"
    chmod +x "$app_dir/share/applications/de.sorcarion.Sotid.desktop"

    # Setup ~/.local directories
    mkdir -p "$HOME/.local/bin"

    # Link the binary
    ln -sf "$app_dir/bin/sotid" "$HOME/.local/bin/sotid"

    # Copy the .desktop file into the user applications dir
    cp "$app_dir/share/applications/de.sorcarion.Sotid.desktop" "$HOME/.local/share/applications/de.sorcarion.Sotid.desktop"
    sed -i "s|Icon=sotid|Icon=$app_dir/share/icons/hicolor/512x512/apps/sotid.png|g" "$HOME/.local/share/applications/de.sorcarion.Sotid.desktop"
    sed -i "s|Exec=sotid|Exec=$app_dir/bin/sotid|g" "$HOME/.local/share/applications/de.sorcarion.Sotid.desktop"
}

macos() {
    # On macOS we just expose the CLI binary; no .app bundle is installed.
    target_dir="target/release"

    mkdir -p "$HOME/.local/bin"
    ln -sf "$PWD/$target_dir/cli" "$HOME/.local/bin/sotid"
}

main "$@"
