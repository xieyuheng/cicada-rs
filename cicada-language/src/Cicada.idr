module Cicada

import Dict

data Exp : Type where
  CallExp
    : (name : String) -> Exp
  LetExp
    : (name_list : List String) -> Exp
  ClosureExp
    : (body : List Exp) -> Exp
  ArrowExp
    : (ante : List Exp) -> (succ : List Exp) -> Exp
  ApplyExp
    : Exp
  CaseExp
    : (arg_list :  List Exp) ->
      (closure_dict : Dict String Exp) -> Exp
  FieldExp
    : (name : String) -> Exp
  ColonExp
    : (name : String) ->
      (type_list : List Exp) -> Exp
  DoubleColonExp
    : (name : String) ->
      (type_list : List Exp) -> Exp
  BeginExp
    : (body : List Exp) -> Exp
  CommaExp
    : Exp
  TypeTTExp
    : Exp

data Den : Type where
  FunDen
    : (name : String) ->
      (type_arrow : Exp) ->
      (body : List Exp) -> Den
  DataConsDen
    : (name : String) ->
      (type_arrow : Exp) ->
      (cons_arrow : Exp) -> Den
  TypeConsDen
    : (name : String) ->
      (type_arrow : Exp) ->
      (cons_arrow : Exp) -> Den
  UnionConsDen
    : (name : String) ->
      (type_arrow : Exp) ->
      (sub_name_list : List String) -> Den

record HypoId where
  constructor MkHypoId
  id : Nat

data Obj : Type where
  DataObj
    : (type : Obj) ->
      (field_dict : Dict String Obj) -> Obj
  DataType
    : (name : String) ->
      (field_dict : Dict String Obj) -> Obj
  UnionType
    : (name : String) ->
      (field_dict : Dict String Obj) -> Obj
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

exe : Env -> Exp -> Env
exe env (CallExp name) = env
exe env (LetExp name_list) = env
exe env (CommaExp) = env
