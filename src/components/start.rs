use std::run::process_status;
pub fn init(p:int)
{
        process_status("sh" , [~"startweb.sh", format!("{}" , p)]);
}
pub fn move()
{
        println("Generating templates");  
        process_status("cp" , [~"-rf", ~"../../templates/", ~"../../.web/"]);

}
pub fn modifify()
{
        process_status("sh" , [~"../../models/.initadd.sh"]);

}



