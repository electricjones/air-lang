# Todo
- Add new operations (* / ++ -- ^ ^x)
- add proper CLI and REPL

- Allow sets `[]`
- Operate on sets `+[1, 3, 4]` = `8`


- labels `a = 2 + 3`
- introduce `;`

- Tie it together
- All lazy evaluated

```air
a = [1, 3, 4, 2 + 4];
b = a^3;
c = a + b + (+[2, 3, b]);
```

https://createlang.rs/01_calculator/vm.html