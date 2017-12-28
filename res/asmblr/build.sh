#! /bin/sh -e
# -e stops it on an error
BLACK=`echo -en '\e[30m'`
RED=`echo -en '\e[31m'`
GREEN=`echo -en '\e[32m'`
YELLOW=`echo -en '\e[33m'`
BLUE=`echo -en '\e[34m'`
MAGENTA=`echo -en '\e[35m'`
CYAN=`echo -en '\e[36m'`
WHITE=`echo -en '\e[37m'`
DEFAULT=`echo -en '\e[39m'`

rm test.hex 2> /dev/null || true
env AS_MSGPATH=msg ./asl -xx -c -q -A test.asm
./s3p2bin test.p test.bin test.h
xxd test.bin > test.hex
rm test.p test.h 2> /dev/null || true

# tools/build_linux/fixpointer sboom.h sboom.bin off_3A294 MapRUnc_Sonic \$2D 0 4
# tools/build_linux/fixheader sboom.bin
