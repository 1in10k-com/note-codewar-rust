1 Reversed sequence
![](images/2021-07-13-15-33-46.png)
![](images/2021-07-13-15-50-17.png)
rust似乎不接受 n-- 这种写法。
![](images/2021-07-13-15-51-36.png)
官方使用了rev方法，应该是倒转顺序的方法。
***
2 Find the smallest integer in the array
![](images/2021-07-13-16-08-31.png)
这是迭代器的用法。
![](images/2021-07-13-16-18-01.png)
这是for循环用法。
![](images/2021-07-13-16-22-40.png)
最佳答案，min获得最小值。unwrap大意是返回some包含的值。
***
3 You Can't Code Under Pressure #1
![](images/2021-07-13-17-04-44.png)
![](images/2021-07-13-17-05-31.png)
***
4 Sort and Star
![](images/2021-07-13-17-07-20.png)
![](images/2021-07-13-17-09-17.png)
这踢较难，以后再做。cccc
***
5 Get Nth Even Number
![](images/2021-07-13-17-12-49.png)
***
6 DNA to RNA Conversion
题目不容易看懂，略
***
7 Invert values
![](images/2021-07-13-19-22-12.png)
![](images/2021-07-13-19-34-46.png)
zzzz 需要复习下最佳答案。
***
8 Can we divide it?
![](images/2021-07-13-19-40-51.png)
注意中间是两个&&
***
9 Count of positives / sum of negatives 
![](images/2021-07-13-22-12-52.png)
这段代码是错的，没考虑input是空的情况。
![](images/2021-07-13-22-17-34.png)
最佳答案。fold方法介绍：https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
***
10 Remove String Spaces
![](images/2021-07-13-22-38-20.png)
replace用去替换字符串的内容。  
split_whitespace()用于以空格为界限分割字符串。
***
