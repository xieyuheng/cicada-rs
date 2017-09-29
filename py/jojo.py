import inspect
import types

class VM:
    def __init__(self, ds, rs):
        self.ds = ds
        self.rs = rs

class JoJo:
    def __init__(self, *body):
        self.length = len(body)
        self.body = list(body)

def exe_one_step(vm):
    jo = vm.rs.pop()
    if isinstance(jo, types.FunctionType):
        parameters = inspect.signature(jo).parameters
        length = len(parameters)
        arguments = []
        i = 0
        while i < length:
            arguments.append(vm.ds.pop())
            i = i + 1
        arguments.reverse()
        result = jo(*arguments)
        vm.ds.append(result)
    else:
        vm.ds.append(jo)

def exe(vm):
    while vm.rs != []:
        exe_one_step(vm)
        print (vm.ds)

class Human:
    species = "H. sapiens"
    def __init__(self, name):
        self.name = name
    def say(self, msg):
        print ("{name}: {message}".format(name=self.name, message=msg))
    def sing(self):
        return 'yo... yo... microphone check... one two... one two...'
    @classmethod
    def get_species(cls):
        return cls.species
    @staticmethod
    def grunt():
        return "*grunt*"

# def k(x, y):
#     pass

# def k1(x, y=1):
#     pass

# print(inspect.signature(k).parameters)

# print(inspect.signature(k1).parameters)
# print(len(inspect.signature(k1).parameters))

# print(inspect.signature(abs).parameters)
# print(inspect.signature(Human.say).parameters)

# def add(x, y):
#     return x + y

# vm = VM([1, 2], JoJo(add, 3, add))

# exe(vm)

# print (type(vm))
# print (type(exe))
