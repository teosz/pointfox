use std::run::process_status;
pub fn init(p:int)
{
        process_status("sh" , [~"startweb.sh", format!("{}" , p)]);
}
pub fn move()
{
        process_status("cp" , [~"-rf", ~"../templates/", ~"../.web/"]);

}




