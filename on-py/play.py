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

# isinstance(h.say, types.LambdaType)


print (type(len))

import sys
import os

print (type(sys.exit))
print (type(sys))

isinstance(Human.say, types.MethodType)

isinstance(sys.exit, types.BuiltinFunctionType)
isinstance(sys.exit, types.BuiltinMethodType)


isinstance(len, types.BuiltinFunctionType)
isinstance(len, types.BuiltinMethodType)

get_signature(exe)
# get_signature(print)
