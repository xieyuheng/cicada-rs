# M-expression in command line

## Usage

```
USAGE:
    mexp [FLAGS] [OPTIONS] <FILE>

FLAGS:
        --self       output to mexp format itself
        --tree       output to tree format
        --compact    compact version of output

OPTIONS:
    -o, --output <FILE>

ARGS:
    <FILE>
```

## Example

- content of `mexp-examples/data.mexp`

```
list-t = data {
  t : type-tt
}
```

- `mexp mexp-examples/data.mexp --tree`

```
mexp:infix {
  mexp:sym {
    "list-t"
  }
  "="
  mexp:apply {
    mexp:sym {
      "data"
    }
    arg:block {
      '{'
      list:cons (mexp) {
        mexp:infix {
          mexp:sym {
            "t"
          }
          ":"
          mexp:sym {
            "type-tt"
          }
        }
        list:null (mexp) {}
      }
      '}'
    }
  }
}
```

- `mexp mexp-examples/data.mexp --tree --compact`

```
mexp:infix { mexp:sym { "list-t" } "=" mexp:apply { mexp:sym { "data" } arg:block { '{' list:cons (mexp) { mexp:infix { mexp:sym { "t" } ":" mexp:sym { "type-tt" } } list:null (mexp) {} } '}' } } }
```
