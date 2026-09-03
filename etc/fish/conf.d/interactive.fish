if status is-interactive
    # Branding di benvenuto bolina ⛵
    function fish_greeting
        if type -q fastfetch
            fastfetch
        end
    end

    # Inizializzazione Starship
    if type -q starship
        starship init fish | source
    end

    # Inizializzazione Zoxide
    if type -q zoxide
        zoxide init fish | source
    end

    # Inizializzazione FZF
    if type -q fzf
        fzf --fish | source
    end

    # Aliases
    alias agy='antigravity'
    alias ls='ls --color=auto'
    alias ll='ls -lh'
    alias la='ls -lAh'
    alias ..='cd ..'
    alias ...='cd ../..'

    # Integrazione VTE (per tracking directory nel terminale)
    if test -f /etc/profile.d/vte.sh; and type -q bass
        bass source /etc/profile.d/vte.sh
    end
end
