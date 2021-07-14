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
11 Difference of Volumes of Cuboids
![](images/2021-07-14-12-02-10.png)
***
12 Difference of Volumes of Cuboids
![](images/2021-07-14-12-09-21.png)  
a,b abs计算绝对值  
c，product计算乘积。
***
13 A wolf in sheep's clothing
![](images/2021-07-14-13-02-12.png)
zzzz string::from 里面不能加变量，format! 可以。
![](images/2021-07-14-13-08-32.png)
rev让迭代器的序列颠倒。position获得元素在序列中的值，由some index,或none保存。unwrap获取some里的值。
***
14 Convert a Boolean to a String
![](images/2021-07-14-13-20-43.png)
![](images/2021-07-14-13-21-34.png)
***
15 The Feast of Many Beasts
![](images/2021-07-14-15-24-52.png)
![](images/2021-07-14-15-27-17.png)
chars() Returns an iterator over the chars of a string slice.  迭代字符。  
last() 返回迭代的最后一个元素。  
nth(n) 返回迭代的第n个元素。
***
16 Convert a Number to a String!
![](images/2021-07-14-15-41-19.png)
![](images/2021-07-14-15-44-20.png)
***
17 Grasshopper - Terminal game move function
没看懂题目。llll
***
18 Reversed Strings
![](images/2021-07-14-15-57-55.png)
![](images/2021-07-14-15-58-46.png)
可以像最佳答案这样用collect::<String>()手动指定类型。
***
19 Is he gonna survive?
![](images/2021-07-14-16-37-32.png)
可能导致乘法溢出，但暂时不考虑。
***
20 Grasshopper - Summation
![](images/2021-07-14-17-13-40.png)
zzzz 目前理解是用vec.iter().sum()，因为iter只能获得引用，但加起来不能输出为引用的结果（因为没有值给它引用），所以会报错。而用vec.into_iter().sum()因为获得了所有权，所以没问题，不用显式指定类型。而用vec.iter().sum::<i32>()，因为指定了输出为i32类型，所以虽然接收的是&i32，但仍然正常输出
![](images/2021-07-14-17-14-32.png)
