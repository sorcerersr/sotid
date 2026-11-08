#!/usr/bin/env sh
set -eu

# Uninstalls Sotid that was installed using the install.sh script.

prompt_remove_preferences() {
    printf "Do you want to keep your Sotid preferences? [Y/n] "
    read -r response
    case "$response" in
        [nN]|[nN][oO])
            rm -rf "$HOME/.config/sotid"
            echo "Preferences removed."
            ;;
        *)
            echo "Preferences kept."
            ;;
    esac
}

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

    "$platform"

    echo "Sotid has been uninstalled"
}

linux() {
    # Remove the app directory
    rm -rf "$HOME/.local/sotid.app"

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/sotid"

    # Remove the .desktop file
    rm -f "$HOME/.local/share/applications/de.sorcarion.Sotid.desktop"

    prompt_remove_preferences
}

macos() {
    # Remove the binary symlink
    rm -f "$HOME/.local/bin/sotid"

    prompt_remove_preferences
}

main "$@"
