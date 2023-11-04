use std::{
    env
,   path::Path, fs, io::prelude::*
};

use lopdf::{
    Result, Document
};



fn main()
-> Result<()>
{
    let fileName = GetPDFFileName() ;
    let mut textExtractor = TextExtractor::newWithSavingFolder( fileName.as_str() ) ;
    let doc = Document::load( fileName )? ;
    for pageId in doc.page_iter() {
        let fonts = doc.get_page_fonts( pageId ) ;
        let content = doc.get_page_content( pageId )? ;
    }
    Ok( () )
}



fn GetPDFFileName()
-> String
{
    let arg1 = env::args().nth( 1 ) ;
    arg1.expect( "PDF file as first argument" )
}



struct TextExtractor
{
    folderName : String
,   file : fs::File
}


impl TextExtractor
{
    fn newWithSavingFolder(
        fileName : &str
    )-> Self
    {
        let folderName = Path::new( fileName ).file_stem().unwrap() ;
        let folder = Path::new( folderName ) ;
        if folder.is_dir() {
            fs::remove_dir_all( folder ) ;
        }
        fs::create_dir( folder ) ;
        let filePath = folder.join( "00.txt" ) ;
        Self {
            folderName : folderName.to_str().unwrap().to_owned()
        ,   file : fs::File::create( &filePath ).unwrap()
        }
    }
}
