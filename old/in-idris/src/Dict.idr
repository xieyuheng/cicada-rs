module Dict

public export
data Dict : (k : Type) -> (v : Type) -> Type where
  EmptyDict : Dict k v
  ExtendDict : k -> v -> Dict k v -> Dict k v

export
find : (Eq k) => k -> Dict k v -> Maybe v
find x EmptyDict = Nothing
find x (ExtendDict y v dict) with (x == y)
  | True = Just v
  | False = find x dict

export
insert : (Eq k) => k -> v -> Dict k v -> Dict k v
insert = ExtendDict
