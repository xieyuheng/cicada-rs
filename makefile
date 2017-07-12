.ONESHELL:
cc = gcc
copy = rsync --recursive --links --perms --times --group --owner --devices --specials --verbose --human-readable
l = -ldl
f = -w -rdynamic -O2
#f = -w -rdynamic

build:
	./tool/tangle.js &&\
	xxd -i core/0.0.1/core.jo > core/0.0.1/core.h &&\
	$(cc) $(f) $(l) jojo.c -o jojo

user-install:
	$(copy) core ~/.jojo/
	$(copy) module ~/.jojo/

clean:
	rm jojo
	rm -f *~ */*~ */*/*~ */*/*/*~ */*/*/*/*~ */*/*/*/*/*~
