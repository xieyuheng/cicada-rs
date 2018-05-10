module Cicada

import Dict

record Env where
  constructor MkEnv
  nameDict : Dict String Den
  dataStack : List Obj
  frameStack : List Frame
  scopeStack : List Scope
  dataBindDict : Dict HypoId Obj
  typeBindDict : Dict HypoId Obj

data Den : Type where

data Obj : Type where
