# External Paths for Fish (Homebrew and Flatpak)

# Homebrew (append to ensure /usr/bin system binaries take precedence)
if test -d /home/linuxbrew/.linuxbrew/bin
    fish_add_path --append /home/linuxbrew/.linuxbrew/bin
    fish_add_path --append /home/linuxbrew/.linuxbrew/sbin
end

# Flatpak (append to preserve system path priority)
if test -d /var/lib/flatpak/exports/bin
    fish_add_path --append /var/lib/flatpak/exports/bin
end

if test -d $HOME/.local/share/flatpak/exports/bin
    fish_add_path --append $HOME/.local/share/flatpak/exports/bin
end
