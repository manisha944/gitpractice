//manisha world hello
//everyone
/*use std::ops;
fn multiply<T:std::ops::Mul<Output = T>>(a:T,b:T)->T{
    a*b
}
fn main()
{
    assert_eq!(6,multiply(2u8,3u8));
    assert_eq!(5.0,multiply(1.0,5.0));
    println!("passed!");
}*/

/*use std::ops;
fn adding<T:std::ops::Add<Output=T>>(a:T,b:T)->T{
    a+b
}
fn main()
{
    let add_num:i32=adding(34,43);
    println!("the nums should be {}",add_num);
}*/
#[derive(Debug,PartialEq,PartialOrd)]
use std::ops;
struct Foo;
struct Bar;

struct BarFoo;
struct FooBar;

impl ops::Add<Bar> for Foo{
    type Output = FooBar;
    fn add(self,_rhs: Bar)->FooBar{
        FooBar
    }
}

impl ops::Sub<Bar> for Foo{
    type Output = BarFoo;
    fn sub(self,_rhs: Bar)->BarFoo{
        BarFoo
    }
}

fn main()
{
    assert_eq!(Foo + Bar,FooBar);
    assert_eq!(Foo - Bar,BarFoo);
    println!("passed!");
}