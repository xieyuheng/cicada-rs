.ONESHELL:

build:
	cc vm1.c -o vm1

run:
	./vm1

dev:
	make clean
	make build

clean:
	@
	echo -e "\e[33;1m"
	echo "* clean"
	echo -e "\e[0m"
	rm vm1
	rm -f *~ */*~ */*/*~ */*/*/*~ */*/*/*/*~  */*/*/*/*/*~
