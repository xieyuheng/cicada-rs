module Dict

public export
data Dict : Type -> Type -> Type where
  EmptyDict : Dict k v
  ExtendDict : k -> v -> Dict k v -> Dict k v
