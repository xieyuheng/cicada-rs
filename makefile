.ONESHELL:

build:
	cc jojo.c -o jojo

run:
	./jojo

dev:
	make clean
	make build

clean:
	@
	echo -e "\e[33;1m"
	echo "* clean"
	echo -e "\e[0m"
	rm jojo
	rm -f *~ */*~ */*/*~ */*/*/*~ */*/*/*/*~  */*/*/*/*/*~
