my custom python lint program, checks list
1. import statements that ends with "\"
2. list, tuple statements thating using implict string concacte
```python
list1 = ["1",
        "2"
        "3"]
```

3. trailing comma in assign statement, make it a tuple
t = "1",

run it with `cargo run -- test_folder/`
