1 Sum of positive
![](images/2021-07-14-17-42-50.png)
![](images/2021-07-14-17-52-09.png)
![](images/2021-07-14-18-13-38.png)
is_positive()方法判断是否为正。
***
2 Function 1 - hello world
![](images/2021-07-14-18-26-06.png)
![](images/2021-07-14-18-26-59.png)
需要返回值而不是打印值。
***
3 Beginner - Reduce but Grow
![](images/2021-07-14-18-52-41.png)
![](images/2021-07-14-18-56-38.png)
***
4 Beginner - Lost Without a Map
![](images/2021-07-14-20-08-35.png)
***
5 Century From Year
![](images/2021-07-14-20-17-31.png)
![](images/2021-07-14-20-19-13.png)
***
6 Do I get a bonus?
![](images/2021-07-14-20-26-20.png)
![](images/2021-07-14-20-27-31.png)
***
7 get character from ASCII Value
![](images/2021-07-14-20-36-22.png)
***
8 L1: Bartender, drinks!
![](images/2021-07-14-20-59-46.png)
match 不能写.to_string()或者类似的函数调用
![](images/2021-07-14-21-00-03.png)
![](images/2021-07-14-21-12-16.png)
也可以这样，不过暂时不懂为什么要用&*而不是&。
***
9 Smallest unused ID
![](images/2021-07-14-21-48-13.png)
这样写能通过测试，但不能通过提交。
![](images/2021-07-14-21-54-00.png)
**skip_while:** Creates an iterator that skips elements based on a predicate. skip_while() takes a closure as an argument. It will call this closure on each element of the iterator, and ignore elements until it returns false After false is returned, skip_while()'s job is over, and the rest of the elements are yielded.    
大意是当闭包返回false时停止遍历，还没遍历的元素会保留。  
zzzz
***
10 Parse nice int from char problem  
![](images/2021-07-14-22-10-05.png) 
这样结果是错误的，不能用as。
![](images/2021-07-14-22-13-43.png)  
a,&str可以用[..1]获得第一个字符。  
b,parse()方法可以转换很多种类型。这里应该是通过函数签名自动推断为u32类型。  
c,to_digit(10)方法把字符转换成10进制数字。
***
11 N-th Power 
![](images/2021-07-16-18-01-56.png)
get方法，pow power n次方方法。
***
12 Filling an array (part 1)
![](images/2021-07-16-18-11-14.png)
![](images/2021-07-16-18-13-40.png)
活用(0 .. n)这种方式。
***
13 Function 2 - squaring an argument 
![](images/2021-07-16-18-20-48.png)
***
13 Dollars and Cents
![](images/2021-07-16-19-09-12.png)
{:02}固定写法，round取与其最接近的整数。
***
14 Beginner Series #2 Clock
![](images/2021-07-16-19-14-06.png)
***
15 Holiday VI - Shark Pontoon
太长不看
***
16 Multiply
![](images/2021-07-16-19-15-47.png)
***
17 Check the exam
![](images/2021-07-16-19-39-06.png)
zzzz  
test能通过，但random不能，不知啥原因。  
![](images/2021-07-16-19-49-23.png)
zip，链接两个迭代器。fold把元素弄进计算器、叠加器。max(n)取值和n中的最大值。
***
18 Switch it Up!
![](images/2021-07-16-19-54-39.png)
![](images/2021-07-16-19-55-44.png)
这个方法不错。
***
19 L1: Set Alarm
![](images/2021-07-16-22-52-13.png)
!表示取反
![](images/2021-07-16-22-53-45.png)
这个不懂 qqqq
***
20 Are You Playing Banjo?
![](images/2021-07-16-23-14-48.png)
这是错误的，因为得到的是option，而不是char。需要使用unwrap或后面介绍的unwrap_or。
![](images/2021-07-16-23-25-36.png)
a, &name[0..1]就能取第一个字符。  
b, starts_with返回布尔值  
c, unwrap_or 用法，似乎必须接单引号。   qqqq