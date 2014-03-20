extern mod std;
mod models;
fn main()
{
    let file = ~"index.html";
    models::add(~"user", ~"Mihai", file);
}
