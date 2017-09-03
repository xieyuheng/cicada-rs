.ONESHELL:
cc = gcc
#cc = clang
copy = rsync --recursive --links --perms --times --group --owner --devices --specials --verbose --human-readable
w = -Wno-int-conversion -Wno-incompatible-pointer-types -Wno-return-type -Wunused-value
o = -O2
f = -rdynamic
l = -ldl

build:
	make tangle
	xxd -i core.jo > core.h
	time $(cc) $(w) $(o) $(f) $(l) jojo.c -o jojo

tangle:
	./tool/tangle.js

# user-install:
#	$(copy) core ~/.jojo/
#	$(copy) module ~/.jojo/

clean:
	rm jojo
	rm -f *~ */*~ */*/*~ */*/*/*~ */*/*/*/*~ */*/*/*/*/*~
