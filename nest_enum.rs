enum Area{
    Circle(i32),
    Rectangle{width:i32,hieght:i32}
}

enum Graphic{
    Text(Area)
}

impl Graphic{
    fn find_area(&self)->i32{
        let num=self.width*self.hieght;
        num
    }
}

fn main()
{
    let area=Graphic::Text(Area::Rectangle{width:3,hieght:4});
    println!("the area of rectangle is {}"area.find_area());
}