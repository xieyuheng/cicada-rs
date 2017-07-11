.ONESHELL:
CC = gcc
COPY = rsync --recursive --links --perms --times --group --owner --devices --specials --verbose --human-readable

build:
	./tool/tangle.js &&\
	xxd -i core/0.0.1/core.jo > core/0.0.1/core.h &&\
	$(CC) -O2 -w -ldl jojo.c -o jojo -rdynamic

user-install:
	$(COPY) core ~/.jojo/
	$(COPY) module ~/.jojo/

clean:
	rm jojo
	rm -f *~ */*~ */*/*~ */*/*/*~ */*/*/*/*~ */*/*/*/*/*~
