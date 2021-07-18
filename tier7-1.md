1 Moves in squared strings (I)
太难，cccc
***
2 Leap Years
![](images/2021-07-13-20-11-42.png)
match 这样用是错的，用最佳答案的方式。
![](images/2021-07-13-20-12-03.png)
***
3 Over The Road
没看懂题目
***
4 Highest and Lowest
```
```
<details>
  <summary><mark><font color=darkred>code</font></mark></summary>
  <p> - 测试 测试测试</p>
  <pre><code>  
fn high_and_low(numbers: &str) -> String {
    let v: Vec<&str> = numbers.split_whitespace().collect();
    let max = v.iter().max().unwrap();
    let min = v.iter().min().unwrap();
    format!("{} {}", max, min)
}
//此答案ramdom_test some_test会报错，其它test不会。
  </code></pre>
</details>