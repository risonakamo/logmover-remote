use logmover_remote::relocation::relocate;

fn main()
{
    relocate(
        r"C:\Users\ktkm\Desktop\logmover-remote\testzone\vids",
        r"C:\Users\ktkm\Desktop\logmover-remote\testzone\delete",
        "something.txt"
    );
}