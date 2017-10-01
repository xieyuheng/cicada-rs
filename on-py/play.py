import inspect
import types

from jojo import *

class Human:
    def __init__(self, name):
        self.name = name
    def say(self, msg):
        print ("{name}: {message}"
               .format(name=self.name,
                       message=msg))
    def sing(self):
        return 'singing'


inspect.signature(Human.say).parameters
h = Human("h1")
inspect.signature(h.say).parameters

isinstance(Human.say, types.FunctionType)
isinstance(Human.say, types.MethodType)
isinstance(Human.say, types.LambdaType)

isinstance(h.say, types.MethodType)
isinstance(h.say, types.MethodType)
isinstance(h.say, types.LambdaType)

import sys
import os

isinstance(Human.say, types.MethodType)

isinstance(sys.exit, types.BuiltinFunctionType)
isinstance(sys.exit, types.BuiltinMethodType)

isinstance(len, types.BuiltinFunctionType)
isinstance(len, types.BuiltinMethodType)

get_signature(exe)
get_signature(print)

def k0(a, b0 = "b0", *b, c0 = "c0", **c):
    print (a)
    print (b0)
    print (b)
    print (c0)
    print (c)

def k1(a, b0 = "b0", *, c0 = "c0", **c):
    print (a)
    print (b0)
    print (c0)
    print (c)

def k2(a, b0 = "b0", *, c0, **c):
    print (a)
    print (b0)
    print (c0)
    print (c)

def report_parameters(parameters):
    for k, v in parameters.items():
        print("- name : {}".format(k))
        print("  kind : {}".format(v.kind))
        print("  default : {}".format(v.default))

p0 = get_signature(k0).parameters
p1 = get_signature(k1).parameters
p2 = get_signature(k2).parameters

report_parameters(p0)
report_parameters(p1)
report_parameters(p2)
