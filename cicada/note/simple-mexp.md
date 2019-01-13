# simple mexp

``` grammar
mexp:str  = gr { quotation ('"') }
mexp:char = gr { quotation ('\'') }
mexp:sym  = gr { symbol? }

mexp:array = gr { '[' list (mexp) ']' }
mexp:arrow = gr { "--" list (mexp) "->" mexp }
mexp:infix = gr { mexp op? mexp }

mexp:dot = gr { head '.' mexp:sym }
mexp:app = gr { head arg }

head:sym = gr { mexp:sym }
head:dot = gr { mexp:dot }
head:app = gr { mexp:app }

arg:block = gr { '{' list (mexp) '}' }
arg:tuple = gr { '(' list (mexp) ')' }

list:null = gr-lambda (t) {}
list:cons = gr-lambda (t) { t list (t) }
```
