import inspect
import types

class RP:
    def __init__(self, jojo):
        self.cursor = 0
        self.length = jojo.length
        self.body = jojo.body
        self.lr = jojo.lr.copy()

class VM:
    def __init__(self, ds, rs):
        self.ds = ds
        self.rs = rs

class LGET:
    def __init__(self, name):
        self.name = name

class LSET:
    def __init__(self, name):
        self.name = name

class JOJO:
    def __init__(self, *body):
        self.length = len(body)
        self.body = list(body)
        self.lr = {}

class MSG:
    def __init__(self, message):
        self.message = message

class CLO:
    def __init__(self, *body):
        self.length = len(body)
        self.body = list(body)
        self.lr = {}

def exe_one_step(vm):
    rp = vm.rs.pop()
    jo = rp.body[rp.cursor]

    # handle tail call
    if rp.cursor >= rp.length - 1:
       pass
    else:
       rp.cursor = rp.cursor + 1
       vm.rs.append(rp)

    # dispatching
    if isinstance(jo, types.LambdaType) \
    or isinstance(jo, types.MethodType) \
    or isinstance(jo, types.BuiltinFunctionType):
        parameters = inspect.signature(jo).parameters
        length = len(parameters)
        arguments = []
        i = 0
        while i < length:
            arguments.append(vm.ds.pop())
            i = i + 1
        arguments.reverse()
        result = jo(*arguments)
        if isinstance(result, tuple):
            vm.ds.extend(result)
        elif result == None:
            pass
        else:
            vm.ds.append(result)
    elif isinstance(jo, JOJO):
        vm.rs.append(RP(jo))
    elif isinstance(jo, LGET):
        value = rp.lr[jo.name]
        vm.ds.append(value)
    elif isinstance(jo, LSET):
        value = vm.ds.pop()
        rp.lr[jo.name] = value
    elif isinstance(jo, MSG):
        o = vm.ds.pop()
        c = type(o)
        f = c.__dict__[jo.message]

        parameters = inspect.signature(f).parameters
        length = len(parameters) - 1
        arguments = []
        i = 0
        while i < length:
            arguments.append(vm.ds.pop())
            i = i + 1
        arguments.reverse()
        result = f(o, *arguments)
        if isinstance(result, tuple):
            vm.ds.extend(result)
        elif result == None:
            pass
        else:
            vm.ds.append(result)

    else:
        vm.ds.append(jo)

def exe(vm):
    while vm.rs != []:
        exe_one_step(vm)
        print (vm.ds)
    print ("- exe end")

ifte = JOJO()

def drop(a):
    return ()

def dup(a):
    return (a, a)

def over(a, b):
    return (a, b, a)

def tuck(a, b):
    return (b, a, b)

def swap(a, b):
    return (b, a)
