import inspect
import types

class RP:
    def __init__(self, fun):
        self.cursor = 0
        self.length = fun.length
        self.body = fun.body
        self.lr = fun.lr.copy()

class VM:
    def __init__(self, ds, rs):
        self.ds = ds
        self.rs = rs

class LGET:
    def __init__(self, name):
        self.name = name

    def jo_exe(self, rp, vm):
        value = rp.lr[self.name]
        vm.ds.append(value)

class LSET:
    def __init__(self, name):
        self.name = name

    def jo_exe(self, rp, vm):
        value = vm.ds.pop()
        rp.lr[self.name] = value

class JOJO:
    def __init__(self, *body):
        self.length = len(body)
        self.body = list(body)
        self.lr = {}

    def jo_exe(self, rp, vm):
        vm.rs.append(RP(self))

class MSG:
    def __init__(self, message):
        self.message = message

    def jo_exe(self, rp, vm):
        o = vm.ds.pop()
        c = type(o)
        fun = getattr(c, self.message)

        parameters = inspect.signature(fun).parameters
        length = len(parameters) - 1
        arguments = []
        i = 0
        while i < length:
            arguments.append(vm.ds.pop())
            i = i + 1
        arguments.reverse()
        result = fun(o, *arguments)
        if isinstance(result, tuple):
            vm.ds.extend(result)
        elif result == None:
            pass
        else:
            vm.ds.append(result)

class CLOSURE:
    def __init__(self, body, lr):
        self.length = len(body)
        self.body = body
        self.lr = lr

class CLO:
    def jo_exe(self, rp, vm):
        body = vm.ds.pop()
        lr = rp.lr
        clo = CLOSURE(body, lr)
        vm.ds.append(clo)

clo = CLO()

class APPLY:
    def jo_exe(self, rp, vm):
        clo = vm.ds.pop()
        vm.rs.append(RP(clo))

apply = APPLY()

class IFTE:
    def jo_exe(self, rp, vm):
        clo2 = vm.ds.pop()
        clo1 = vm.ds.pop()
        test = vm.ds.pop()
        if test:
            vm.rs.append(RP(clo1))
        else:
            vm.rs.append(RP(clo2))

ifte = IFTE()

def exe_fun(fun, rp, vm):
    parameters = inspect.signature(fun).parameters
    length = len(parameters)
    arguments = []
    i = 0
    while i < length:
        arguments.append(vm.ds.pop())
        i = i + 1
    arguments.reverse()
    result = fun(*arguments)
    if isinstance(result, tuple):
        vm.ds.extend(result)
    elif result == None:
        pass
    else:
        vm.ds.append(result)

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
    if isinstance(jo, types.BuiltinFunctionType):
        print ("- exe_one_step fail")
        print ("  meet built in function")

    elif isinstance(jo, types.LambdaType) \
    or isinstance(jo, types.MethodType):
        exe_fun(jo, rp, vm)

    elif hasattr(jo, "jo_exe"):
        jo.jo_exe(rp, vm)

    else:
        vm.ds.append(jo)

def exe(vm):
    while vm.rs != []:
        exe_one_step(vm)
        print (vm.ds)
    print ("- exe end")

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
