use clipboard_win::get_clipboard_string;

fn main()
{
    let a:String=get_clipboard_string().unwrap();
    println!("{}",a);
}