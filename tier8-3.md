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
