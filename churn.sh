set -x

PACKAGE=$1
pacman -S $PACKAGE

# TODO: arg -> package magic happens here

shift
"$@"

# logging happens here

pacman -R $PACKAGE
