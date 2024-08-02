#!/bin/bash

set -exo pipefail

######### VARIABLES #########
INSTALL_DIR="/opt/rmon"
SCRIPT_WORKDIR="/tmp/rmon-installer"
SRC_REMOTE="https://gitlab.com/victordejong/rmon.git"
#############################

# Check installer requirements
SCRIPT_REQ="ansible ansible-playbook git"
command -v ${SCRIPT_REQ} > /dev/null 2>&1

script_help () {
    echo "
        Unsupported argument. The following arguments are supported:
        ${0} [install | uninstall | install-src] [client | server]
    "
    exit 1
}

run () {
    mkdir -p "${SCRIPT_WORKDIR}" && cd "${SCRIPT_WORKDIR}"
    git clone "${SRC_REMOTE}" && cd "${SCRIPT_WORKDIR}"/rmon/ansible

    # Run client-playbook.yaml or server-playbook.yaml
    ansible-playbook --connection=local -t "${1}" "${2}"-playbook.yaml

}

# Validate arguments and run run()
main () {
    case ${1} in

        "install" | "uninstall" | "install-src")
            echo "test"
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
