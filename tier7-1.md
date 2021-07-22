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
<details>
  <summary><mark><font color=darkred>code</font></mark></summary>
  <pre><code>  
fn high_and_low(numbers: &str) -> String {
    let v: Vec<&str> = numbers.split_whitespace().collect();
    let max = v.iter().max().unwrap();
    let min = v.iter().min().unwrap();
    format!("{} {}", max, min)
}
//此答案ramdom_test some_test会报错，其它test不会。
//zzzz cccc
  </code></pre>
</details>

***
5 Powers of i   
看不懂题目  
***
6 String ends with?
![](images/2021-07-18-11-02-30.png)  
![](images/2021-07-18-11-28-36.png)
ends_with看后缀是否满足要求。
***
7 Odd or Even?
![](images/2021-07-18-11-36-08.png)
.
![](images/2021-07-18-11-37-36.png)
似乎不用考虑vec为空的情况？qqqq。  
注意第二种方式可以用一行代码就做完。
***
8 Floating-point Approximation (III)  
太长 llll
***
9 Robinson Crusoe
llll
***
10 How Green Is My Valley?

***
11 Growth of a Population

***
12 Build a square
![](images/2021-07-18-14-39-07.png)
repeat重复元素  
join，在元素间添加其它元素  
zzzz
***
13 Printer Errors
llll
***
14 Looking for a benefactor
llll
***
15 Alphabet symmetry
![](images/2021-07-18-15-29-55.png)
zzzz
***
16 Valid Spacing
![](images/2021-07-22-17-31-24.png)
char怎么转换为&str？ qqqq   
![](images/2021-07-22-17-45-55.png)
starts_with  ends_with  contains



