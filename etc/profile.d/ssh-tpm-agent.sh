# Use ssh-tpm-agent socket when available and SSH_AUTH_SOCK is not already set (e.g. ssh -A)
if [ -z "${SSH_AUTH_SOCK:-}" ] && [ -n "${XDG_RUNTIME_DIR:-}" ] && [ -S "${XDG_RUNTIME_DIR}/ssh-tpm-agent.sock" ]; then
    export SSH_AUTH_SOCK="${XDG_RUNTIME_DIR}/ssh-tpm-agent.sock"
fi
