#!/bin/bash

set -eo pipefail

ve() {
    local py=${1:-python3.8}

    # If not already in virtualenv
    # $VIRTUAL_ENV is being set from $venv/bin/activate script
	  if [ -z "${VIRTUAL_ENV}" ]; then
        if [ ! -d "./.venv" ]; then
            echo "Creating and activating virtual environment ./.venv"
            ${py} -m venv "./.venv" --system-site-package
            echo "export PYTHON=${py}" >> "./.venv/bin/activate"    # overwrite ${python} on .zshenv
            source "./.venv/bin/activate"
            echo "Upgrading pip"
        else
            echo "Virtual environment  ./.venv already exists, updating and activating..."
            source "./.venv/bin/activate"
        fi
        ${py} -m pip install --upgrade pip
        pip install -U -r requirements.txt
    else
        echo "Already in a virtual environment!"
    fi
}

# Activate virtualenv
ve python3

# Linting
# Molecule does not do linting anymore by default:
# https://github.com/ansible/molecule/discussions/3914
yamllint .
ansible-lint

molecule test

echo "-== Test successful! ==-"
