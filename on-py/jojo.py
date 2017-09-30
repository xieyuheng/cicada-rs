import inspect
import types

def get_signature(fun):
    try:
        return inspect.signature(fun)
    except ValueError:
        return False

def fun_p(x):
    if isinstance(x, types.LambdaType):
        return True
    elif isinstance(x, types.MethodType):
        return True
    else:
        return False

def class_p(x):
    if not inspect.isclass(x):
        return False
    elif x == type:
        return False
    else:
        return True

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

def push_result_to_vm(result, vm):
    if isinstance(result, tuple):
        vm.ds.extend(result)
    elif result == None:
        pass
    else:
        vm.ds.append(result)

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
        fun = getattr(o, self.message)

        exe_jo(fun, rp, vm)

class CLOSURE:
    def __init__(self, body, lr):
        self.length = len(body)
        self.body = body
        self.lr = lr

class CLO:
    @classmethod
    def jo_exe(cls, rp, vm):
        body = vm.ds.pop()
        lr = rp.lr
        clo = CLOSURE(body, lr)
        vm.ds.append(clo)

class APPLY:
    @classmethod
    def jo_exe(cls, rp, vm):
        clo = vm.ds.pop()
        vm.rs.append(RP(clo))

class IFTE:
    @classmethod
    def jo_exe(self, rp, vm):
        clo2 = vm.ds.pop()
        clo1 = vm.ds.pop()
        test = vm.ds.pop()
        if test:
            vm.rs.append(RP(clo1))
        else:
            vm.rs.append(RP(clo2))

class NEW:
    @classmethod
    def jo_exe(self, rp, vm):
        c = vm.ds.pop()
        if not class_p(c):
            print ("- NEW.jo_exe fail")
            print ("  argument is not a class : {}".format(c))
        exe_fun(c, vm)

def exe(vm):
    while vm.rs != []:
        exe_one_step(vm)
        print (vm.ds)
    print ("- exe end")

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
    exe_jo(jo, rp, vm)

def exe_jo(jo, rp, vm):
    if fun_p(jo):
        exe_fun(jo, vm)
    elif hasattr(jo, "jo_exe"):
        jo.jo_exe(rp, vm)
    else:
        vm.ds.append(jo)

def exe_fun(fun, vm):
    signature = get_signature(fun)
    if not signature:
        print ("- exe_fun fail to get signature")
        print ("  fun : {}".format(fun))

    parameters = signature.parameters
    length = len(parameters)
    arguments = []
    i = 0
    while i < length:
        arguments.append(vm.ds.pop())
        i = i + 1
    arguments.reverse()

    result = fun(*arguments)

    push_result_to_vm(result, vm)

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
