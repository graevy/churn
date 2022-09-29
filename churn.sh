PACKAGE=`pkgfile -qb $1`

#if "-v" in @; do
#	echo "Installing $PACKAGE...\n"

pacman -S --noconfirm $PACKAGE 1>/dev/null

"$@"

# TODO: logging happens here

#if "-v" in @; do
#	echo "\n Uninstalling $PACKAGE..."
pacman -R --noconfirm $PACKAGE 1>/dev/null
