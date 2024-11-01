#!/bin/bash

VSCODE_WS="$1"
PROJECT="$2"
SSH_REMOTE="$3"
GDBPORT="$4"

TARGET_USER="pi"
ROOT_USER="root"

APP="app"
TARGET_ARCH="aarch64-unknown-linux-gnu"

BUILD_BIN_FILE="${VSCODE_WS}/target/${TARGET_ARCH}/debug/${APP}"
RESOURCES_FOLDER="${VSCODE_WS}/app/resources"

TARGET_FOLDER="/home/${TARGET_USER}/Documents/bin/${PROJECT}"
TARGET_BIN_FILE="${TARGET_FOLDER}/${APP}"

ssh "${ROOT_USER}@${SSH_REMOTE}" "killall -9 gdbserver"

# Copy bin file
if ! rsync -avz "${BUILD_BIN_FILE}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_BIN_FILE}"; then
    # If rsync doesn't work, it may not be available on target. Fallback to trying SSH copy.
    if ! scp "${BUILD_BIN_FILE}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_BIN_FILE}"; then
        exit 2
    fi
fi

# Copy resources folder into resources folder
if ! rsync -avzr "${RESOURCES_FOLDER}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_FOLDER}"; then
    # If rsync doesn't work, it may not be available on target. Fallback to trying SSH copy.
    if ! scp -r "${RESOURCES_FOLDER}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_FOLDER}"; then
        exit 2
    fi
fi


# ssh -f "${TARGET_USER}@${SSH_REMOTE}" "sh -c 'cd ${TARGET_BIN_FOLDER}; nohup gdbserver *:${GDBPORT} ${TARGET_BIN_FILE} > /dev/null 2>&1 &'"

# send print text into tty1, we need root permission to do this
ssh -f "${ROOT_USER}@${SSH_REMOTE}" "sh -c 'cd ${TARGET_FOLDER}; gdbserver *:${GDBPORT} ${TARGET_BIN_FILE} > /dev/tty1'"