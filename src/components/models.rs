use std::run::process_status;

pub fn add(name:  ~str, value: ~str, file: ~str)
{
    process_status("sh", [~"../../models/.replace.sh", name,value,file]);     
}

