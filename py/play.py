import inspect
import types

from jojo import *

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

def add(x, y):
    return x + y

vm = VM(
    [1, 2],
    [RP(JOJO(add, 3, add, 1, 2, swap, dup, dup, drop, drop))]
)

exe(vm)

def k(a, *b, **c):
    print (a)
    print (b)
    print (c)

def write(string):
    print(string)

h = Human("hhh")

p1 = JOJO(
    1, 2, 3, add, add, write,

#     "k",
#     ["k1", "k2", "k3"],
#     {1: "k1", 2: "k2", 3: "k3"},
#     k,

    Human("kkk"), LSET("h"),

    LGET("h"), "kkk took my baby away", Human.say,

    h, "kkk took my baby away", Human.say,

    "kkk took my baby away", h.say,

    "kkk took my baby away", LGET("h"), MSG("say"),
)

exe(VM([], [RP(JOJO(p1))]))


p2 = JOJO(
    5, LSET("1"),
    100,
    [LSET("2"), LGET("2"), LGET("2"), add, LGET("1"), add], INS_CLO,
    INS_APPLY
)

exe(VM([], [RP(JOJO(p2))]))

inspect.signature(Human.say).parameters
inspect.signature(h.say).parameters

isinstance(Human.say, types.FunctionType)
isinstance(Human.say, types.MethodType)
isinstance(Human.say, types.LambdaType)

isinstance(h.say, types.MethodType)
isinstance(h.say, types.MethodType)
isinstance(h.say, types.LambdaType)
