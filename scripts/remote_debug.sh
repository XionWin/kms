#!/bin/bash

VSCODE_WS="$1"
SSH_REMOTE="$2"
GDBPORT="$3"

PROJECT="kms"
APP="app"
TARGET_ARCH="aarch64-unknown-linux-gnu"
BUILD_BIN_FILE="${VSCODE_WS}/target/${TARGET_ARCH}/debug/${APP}"
TARGET_USER="pi"
TARGET_BIN_FOLDER="/home/${TARGET_USER}/documents/bin/${PROJECT}"
TARGET_BIN_FILE="${TARGET_BIN_FOLDER}/${APP}"

ssh "${TARGET_USER}@${SSH_REMOTE}" "killall gdbserver ${APP}"

if ! rsync -avz "${BUILD_BIN_FILE}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_BIN_FILE}"; then
    # If rsync doesn't work, it may not be available on target. Fallback to trying SSH copy.
    if ! scp "${BUILD_BIN_FILE}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_BIN_FILE}"; then
        exit 2
    fi
fi

# ssh -f "${TARGET_USER}@${SSH_REMOTE}" "sh -c 'cd ${TARGET_BIN_FOLDER}; nohup gdbserver *:${GDBPORT} ${TARGET_BIN_FILE} > /dev/null 2>&1 &'"

# send print text into tty1, we need root permission to do this
ssh -f "root@${SSH_REMOTE}" "sh -c 'cd ${TARGET_BIN_FOLDER}; gdbserver *:${GDBPORT} ${TARGET_BIN_FILE} > /dev/tty1'"