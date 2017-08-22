.ONESHELL:
cc = gcc
#cc = clang
copy = rsync --recursive --links --perms --times --group --owner --devices --specials --verbose --human-readable
w = -Wno-int-conversion -Wno-incompatible-pointer-types -Wno-return-type -Wunused-value
o = -O1
f = -rdynamic
l = -ldl

build:
	./tool/tangle.js
	xxd -i core.jo > core.h 
	$(cc) $(w) $(o) $(f) $(l) jojo.c -o jojo

user-install:
	$(copy) core ~/.jojo/
	$(copy) module ~/.jojo/

clean:
	rm jojo
	rm -f *~ */*~ */*/*~ */*/*/*~ */*/*/*/*~ */*/*/*/*/*~
