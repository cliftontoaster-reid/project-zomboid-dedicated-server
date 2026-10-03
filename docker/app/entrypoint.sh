#!/bin/sh
set -e

INSTDIR="$(dirname $0)"
cd "${INSTDIR}"
INSTDIR="$(pwd)"
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH}:${INSTDIR}/linux64:${INSTDIR}"

exec "$@"
