pub mod fnbox;
 
use fnbox::FnBox;

type Job = Box<dyn FnBox + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}
