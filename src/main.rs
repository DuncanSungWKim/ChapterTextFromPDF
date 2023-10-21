use std::{
    env
};



fn main() {
    let fileName = GetPDFFileName() ;
    println!("Hello, {}!", fileName );
}



fn GetPDFFileName()
-> String
{
    let arg1 = env::args().nth( 1 ) ;
    arg1.expect( "PDF file as first argument" )
}
