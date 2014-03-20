//! A not-quite-trivial HTTP server which responds to requests by showing the request and response
//! headers and any other information it has.

#[crate_id = "info"];

extern mod extra;
extern mod http;
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::Writer;
use extra::time;
use std::io::File;
use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;


#[deriving(Clone)]
struct InfoServer;

impl Server for InfoServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 2020 } }
    }

    fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
         
        w.headers.date = Some(time::now_utc());
        w.headers.content_type = Some(MediaType {
            type_: ~"text",
            subtype: ~"html",
            parameters: ~[(~"charset", ~"UTF-8")]
        });
        let mut s =  format!("{:?}" , r.request_uri).replace("AbsolutePath(~\"/" , "").replace("\")", "");
        if s == ~""
        {
             s = ~"index.html";
        }
        
        let contents =  File::open(&Path::new("../../.web/templates/"+s)).read_to_end();
       
        w.headers.server = Some(~"Pointfox/0.1-pre");
         
   
     
        w.write(contents);
           }
}

fn main() {
    print("Running server on http://localhost:2020");
    InfoServer.serve_forever();
}
