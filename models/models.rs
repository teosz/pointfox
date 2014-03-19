use std::run::process_status;

pub fn add(name:  ~str, value: ~str)
{
    print(name);
    print(value);
    process_status("sh", [~".replace.sh", name,value]); 
    
}

