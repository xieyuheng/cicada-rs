.ONESHELL:
CC = gcc
# CC = clang

build:
	$(CC) -O2 -w -ldl jojo.c -o jojo -rdynamic

play:
	$(CC) -w -ldl jojo.c -o jojo -rdynamic
	./tangle.js
	./jojo core.org

clean:
	@
	echo -e "\e[33;1m"
	echo "* clean"
	echo -e "\e[0m"
	rm jojo
	rm -f *~ */*~ */*/*~ */*/*/*~ */*/*/*/*~ */*/*/*/*/*~
