#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(Debug)]
struct Thing
{
    field:i32
}

fn takesThing(a:&mut Thing)
{
    println!("{:#?}",a);
    a.field=5;
}

fn takesThingNormal(a:Thing)
{

}

fn main()
{
    let list:Vec<i32>=vec![
        1,2,3
    ];

    let list2:Vec<i32>=list.iter().map(|x:&i32|->i32 {
        return x+2;
    }).collect();

    let list3:Vec<i32>=list.into_iter().map(|x:i32|->i32 {
        return x+2;
    }).collect();
}