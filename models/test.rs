extern mod std;
extern mod extra;
use extra::time;

mod models;
fn main()
{
    let file = ~"test.html";
    models::add(~"time", time::strftime("%h %d", &time::now_utc()), file);
}
