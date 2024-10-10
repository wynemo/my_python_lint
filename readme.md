my custom python lint program, checks list
1. import statements that ends with "\\"
2. list, tuple statements thating using implict string concacte
```python
list1 = ["1",
        "2"
        "3"]
```

3. trailing comma in assign statement, make it a tuple
t = "1",

run it with `cargo run -- test_folder/`

我的自定义 Python 代码检查程序，检查列表如下

1. 导入语句以 “\” 结尾的情况
2. 列表和元组语句中使用隐式字符串连接的情况
3. 赋值语句中的尾随逗号会将其转换为元组的情况

[youtube 油管讲解](https://www.youtube.com/watch?v=IWlGx5lMlPA)
