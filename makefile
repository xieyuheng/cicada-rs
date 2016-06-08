.ONESHELL:
CC = gcc
#CC = clang

build:
	$(CC) -ldl jojo.c -o jojo -rdynamic

tangle:
	./tangle.el

run:
	./jojo

play: tangle build run

clean:
	@
	echo -e "\e[33;1m"
	echo "* clean"
	echo -e "\e[0m"
	rm jojo
	rm -f *~ */*~ */*/*~ */*/*/*~ */*/*/*/*~  */*/*/*/*/*~
