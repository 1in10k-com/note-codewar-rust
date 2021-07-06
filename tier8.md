Get the mean of an array
```
fn get_average(marks: &[f32]) -> f32 {
    let mut sum:f32 = 0.;
    for ele in marks.iter(){
        sum = sum + ele;
    }
    let mut ave = (sum/marks.len() as f32).floor();
    ave
}
//floor() 方法执行的是向下取整计算,它返回的是小于或等于函数参数,并且与之最接近的整数。
//as f32 转换类型
//f32 类型需加小数点
```