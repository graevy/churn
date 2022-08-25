PACKAGE=`pkgfile -qb $1`
pacman -Sq --noconfirm $PACKAGE 1>/dev/null

"$@"

# TODO: logging happens here

pacman -R --noconfirm $PACKAGE 1>/dev/null
