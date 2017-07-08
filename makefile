.ONESHELL:
CC = gcc
# CC = clang

build:
	@
	./tangle.js &&\
	echo "- xxd : core.jo -> core.h"  &&\
	xxd -i core.jo > core.h &&\
	echo "- compile : jojo.c -> jojo"  &&\
	$(CC) -w -O2 -ldl jojo.c -o jojo -rdynamic &&\
	echo "- finish ^-^"

clean:
	@
	rm jojo
	rm -f *~ */*~ */*/*~ */*/*/*~ */*/*/*/*~ */*/*/*/*/*~
