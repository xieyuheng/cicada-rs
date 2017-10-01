import inspect
import types

from jojo import *


def test_0():
    vm = exe(VM(
        [1, 2],
        [RP(JOJO(add, 3, add,
                 1, 2, swap, dup, dup, drop))]
    ))
    assert vm.ds == [6, 2, 1, 1]

class Human:
    def __init__(self, name):
        self.name = name
    def say(self, msg):
        print ("{name}: {message}"
               .format(name=self.name,
                       message=msg))
    def sing(self):
        return 'singing'

def test_1():
    jojo = JOJO(
        "xieyuheng", Human, NEW, LSET("h"),
        "kkk took my baby away", LGET("h"), MSG("say"),
        LGET("h"), MSG("name"),
    )
    vm = exe(VM([],
                [RP(JOJO(jojo))]
    ))
    assert vm.ds == ["xieyuheng"]


def test_2():
    jojo = JOJO(
        5, LSET("1"),
        100,
        [LSET("2"), LGET("2"), LGET("2"), add, LGET("1"), add],
        CLO,
        APPLY
    )
    vm = exe(VM([],
                [RP(JOJO(jojo))]
    ))
    assert vm.ds == [205]


def test_3():
    jojo = JOJO(
        False,
        ["true"], CLO,
        ["false"], CLO,
        IFTE
    )
    vm = exe(VM([],
                [RP(JOJO(jojo))]
    ))
    assert vm.ds == ["false"]


def test_4():
    def k(a, b = "b0", *arg_list, k0 = "default k0", **arg_dict):
        return [
            a,
            b,
            arg_list,
            k0,
            arg_dict,
        ]
    jojo = JOJO(
        "aaa",
        "bbb",
        ["k1", "k2", "k3"],
        {"1": "k1", "2": "k2", "3": "k3"},
        k,
    )
    vm = exe(VM([],
                [RP(JOJO(jojo))]
    ))
    ds1 = [[
        'aaa',
        'bbb',
        ('k1', 'k2', 'k3'),
        'default k0',
        {'1': 'k1', '2': 'k2', '3': 'k3'}
    ]]
    assert vm.ds == ds1
