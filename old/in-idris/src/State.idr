module State

public export
record State (s : Type) (a : Type) where
  constructor MkState
  g : (s -> (a, s))

Functor (State s) where
  -- map : (f : a -> b) -> State s a -> State s b
  map f (MkState g) =
    MkState (\ s =>
      let (a, s') = g s
      in ((f a), s'))

Functor (State s) =>
Applicative (State s) where
  -- (<*>) : State s (a -> b) -> State s a -> State s b
  (MkState f) <*> (MkState g) =
    MkState (\ s =>
      let (f', s') = f s
          (a, s'') = g s'
      in (f' a, s''))
  -- pure : a -> State s a
  pure a = MkState (\ s => (a, s))

Applicative (State s) =>
Monad (State s) where
  -- (>>=) : State s a -> (a -> State s b) -> State s b
  (MkState g) >>= m =
    MkState (\ s =>
      let (a, s') = g s
          MkState f = m a
      in f s')
