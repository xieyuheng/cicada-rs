module Cicada

import Dict
import State

data Exp : Type where
  CallExp
    : (name : String) -> Exp
  LetExp
    : (nameList : List String) -> Exp
  ClosureExp
    : (body : List Exp) -> Exp
  ArrowExp
    : (ante : List Exp) ->
      (succ : List Exp) -> Exp
  ApplyExp
    : Exp
  CaseExp
    : (argList :  List Exp) ->
      (closureLict : Dict String Exp) -> Exp
  FieldExp
    : (name : String) -> Exp
  ColonExp
    : (name : String) ->
      (typeList : List Exp) -> Exp
  DoubleColonExp
    : (name : String) ->
      (typeList : List Exp) -> Exp
  BeginExp
    : (body : List Exp) -> Exp
  CommaExp
    : Exp
  TypeTTExp
    : Exp

data Den : Type where
  FunDen
    : (name : String) ->
      (typeArrow : Exp) ->
      (body : List Exp) -> Den
  DataConsDen
    : (name : String) ->
      (typeArrow : Exp) ->
      (consArrow : Exp) -> Den
  TypeConsDen
    : (name : String) ->
      (typeArrow : Exp) ->
      (consArrow : Exp) -> Den
  UnionConsDen
    : (name : String) ->
      (typeArrow : Exp) ->
      (subNameList : List String) -> Den

record HypoId where
  constructor MkHypoId
  id : Nat

data Obj : Type where
  DataObj
    : (type : Obj) ->
      (fieldDict : Dict String Obj) -> Obj
  DataType
    : (name : String) ->
      (fieldDict : Dict String Obj) -> Obj
  UnionType
    : (name : String) ->
      (fieldDict : Dict String Obj) -> Obj
  TypeType
    : (level : Nat) -> Obj
  ClosureObj
    : (scope : Dict String Obj) ->
      (body : List Exp) -> Obj
  ArrowType
    : (ante : List Obj) -> (succ : List Obj) -> Obj
  DataHypo
    : HypoId -> Obj
  TypeHypo
    : HypoId -> Obj

data Frame : Type where
  ScopingFrame
    : (body : List Exp) -> (index : Nat) -> Frame
  SimpleFrame
    : (body : List Exp) -> (index : Nat) -> Frame

record Env where
  constructor MkEnv
  idCounter : Nat
  nameDict : Dict String Den
  dataStack : List Obj
  frameStack : List Frame
  scopeStack : List (Dict String Obj)
  dataBindDict : Dict HypoId Obj
  typeBindDict : Dict HypoId Obj

exe : (exp : Exp) -> State Env a
exe (CallExp name) = ?exe_rhs_1
exe (LetExp nameList) = ?exe_rhs_2
exe (ClosureExp body) = ?exe_rhs_3
exe (ArrowExp ante succ) = ?exe_rhs_4
exe ApplyExp = ?exe_rhs_5
exe (CaseExp argList closureLict) = ?exe_rhs_6
exe (FieldExp name) = ?exe_rhs_7
exe (ColonExp name typeList) = ?exe_rhs_8
exe (DoubleColonExp name typeList) = ?exe_rhs_9
exe (BeginExp body) = ?exe_rhs_10
exe CommaExp = ?exe_rhs_11
exe TypeTTExp = ?exe_rhs_12
