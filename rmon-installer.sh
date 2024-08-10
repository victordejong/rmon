#!/bin/bash
# shellcheck disable=SC1091

set -eo pipefail

######### VARIABLES #########
INSTALL_DIR="/opt/rmon"
SCRIPT_WORKDIR="/tmp/rmon-installer"
SRC_REMOTE="https://gitlab.com/victordejong/rmon.git"
#############################


######### FUNCTIONS #########
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

script_help () {
    echo "
        Unsupported argument. The following arguments are supported:
        ${0} [install | uninstall | install-src] [client | server]
    "
    exit 1
}

run () {
    (mkdir -p "${SCRIPT_WORKDIR}" || true) && cd "${SCRIPT_WORKDIR}"

    (git clone "${SRC_REMOTE}" || true) && cd "${SCRIPT_WORKDIR}"/rmon/ansible

    # Activate venv
    ve python3 > /dev/null 2>&1

    # Run client-playbook.yaml or server-playbook.yaml
    ansible-playbook --connection=local -i localhost, -t "${1}" main.yaml -e variant="${2}"

}
#############################

# Validate requirements, arguments and run run()
main () {

    # Check installer requirements
    SCRIPT_REQ="python python3-venv git"

    # shellcheck disable=SC2086
    command -v ${SCRIPT_REQ} > /dev/null 2>&1

    case ${1} in

        "install" | "uninstall" | "install-src")
            true
        ;;

        *)
            script_help
        ;;
    esac

    case ${2} in

        "client" | "server")
            true
        ;;

        *)
            script_help
        ;;
    esac

    run "${@}"

    exit 0
}

main "${@}"
