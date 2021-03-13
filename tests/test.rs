#![allow(dead_code)]
#![allow(unused_variables)]

struct Thing
{
    items:Vec<Item>
}

struct Item
{
    field:String
}

fn takesThings(items:&Vec<Item>)
{

}

fn main()
{
    let thing=Thing {
        items:vec![
            Item {
                field:"a".to_string()
            }
        ]
    };

    takesThings(&thing.items);

    let thing2:Vec<String>=thing.items.iter().map(|x:&Item|->String {
        return x.field.to_string();
    }).collect();

    takesThings(&thing.items);
}