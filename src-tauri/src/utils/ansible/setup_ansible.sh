#!/bin/bash

# Detect the operating system
OS=$(uname)

# Determine the Python command
if command -v python3 &>/dev/null; then
    PYTHON_CMD="python3"
elif command -v python &>/dev/null; then
    PYTHON_CMD="python"
else
    echo "Python is not installed. Please install Python to proceed."
    exit 1
fi

# macOS specific setup
if [[ "$OS" == "Darwin" ]]; then
    echo "Detected macOS."

    # Check if pipx is installed
    if ! command -v pipx &>/dev/null; then
        echo "pipx is not installed. Please install pipx using 'brew install pipx' and ensure it's in your PATH."
        exit 1
    fi

    # Ensure pipx is set up
    pipx ensurepath

    # Create a virtual environment
    echo "Creating a virtual environment..."
    $PYTHON_CMD -m venv ansible_env
    source ansible_env/bin/activate

    # Install Ansible using pipx
    echo "Installing Ansible using pipx..."
    pip install ansible

    # Deactivate the environment
    deactivate
# Linux specific setup
elif [[ "$OS" == "Linux" ]]; then
    echo "Detected Linux."

    # Check if virtualenv is installed
    if ! $PYTHON_CMD -m pip show virtualenv &>/dev/null; then
        echo "virtualenv is not installed. Installing it now..."
        $PYTHON_CMD -m pip install --user virtualenv
    fi

    # Create a virtual environment and activate it
    echo "Creating a virtual environment..."
    $PYTHON_CMD -m virtualenv ansible_env
    source ansible_env/bin/activate

    # Install Ansible
    echo "Installing Ansible using pip..."
    pip install ansible

else
    echo "Unsupported operating system: $OS"
    exit 1
fi

echo "Ansible environment setup is complete."
