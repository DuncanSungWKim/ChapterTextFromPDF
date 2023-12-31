#![ allow( non_snake_case ) ]

use std::{
    env
,   path::Path, fs, io::prelude::*
};
use std::collections::BTreeMap ;

use lopdf::{
    Result, Document
,   Dictionary, content::Content, Object
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
        textExtractor.AddPage( &fonts, &Content::decode(&content)? )? ;
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
,   text : String
,   canWrite : bool
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
        ,   text : String::new()
        ,   canWrite : false
        }
    }


    fn AddPage( &mut self
    ,   fonts : &BTreeMap<Vec<u8>,&Dictionary>
    ,   content : &Content
    )-> Result< () >
    {
        let mut currentEncoding = None ;
        for operation in &content.operations {
            match operation.operator.as_str() {
                "Tf" =>{
                    let fontName = operation.operands[0].as_name()? ;
                    if let Some(font) = fonts.get( &fontName.to_vec() ) {
                        currentEncoding = Some( font.get_font_encoding() ) ;
                    }
                },
                "Tj" | "TJ" =>{
                    self.CollectText( currentEncoding, &operation.operands ) ;
                },
                "ET" =>{
                    if ! self.text.ends_with( '\n' ) {
                        self.text.push( '\n' ) ;
                    }
                },
                _=>{}
            }
        }
        self.ProcessPage() ;
        self.text.clear() ;
        Ok( () )
    }


    fn CollectText( &mut self
    ,   encoding : Option<&str>
    ,   operands: &[Object]
    )
    {
        for operand in operands.iter() {
            match *operand {
                Object::String( ref bytes, _ ) =>{
                    let text = Document::decode_text( encoding, bytes ) ;
                    self.text.push_str( &text ) ;
                },
                Object::Array( ref array ) =>{
                    self.CollectText( encoding, array ) ;
                    self.text.push( ' ' ) ;
                },
                Object::Integer( i ) =>{
                    if i < -100 {
                        self.text.push( ' ' ) ;
                    }
                }
                _=>{}
            }
        }
    }


    fn ProcessPage( &mut self,
    )
    {
        self.SplitIntoChapters() ;
        if ! self.canWrite  {
            return ;
        }
        self.file.write_all( self.text.as_bytes() ) ;
    }


    fn SplitIntoChapters( &mut self
    )
    {
        static CHAPTER_STARTER : &str = "Chapter " ;

        if self.text.starts_with( "Introduction" ) {
            self.canWrite = true ;
        }
        else if self.text.starts_with( CHAPTER_STARTER ) {
            let beginning = CHAPTER_STARTER.len() ;
            let digits = &self.text[ beginning .. beginning+2 ] ;
            let number = if let Ok( i ) = digits.parse::<i32>() {
                i
            } else {
                digits.chars().nth(0).unwrap() as i32 - '0' as i32
            };
            let fileName = format!( "{:02}", number ) + ".txt" ;
            self.CreateFile( &fileName ) ;
        }
        else if self.text.starts_with( "Appendix" ) {
            self.CreateFile( "A.txt" ) ;
        }
        else if self.text.starts_with( "Part " ) {
            self.canWrite = false ;
        }
        else if self.text.starts_with( "Index" ) {
            self.canWrite = false ;
        }
    }


    fn CreateFile( &mut self,
        fileName : &str
    )
    {
        let filePath = Path::new( &self.folderName ).join( fileName ) ;
        self.file = fs::File::create( &filePath ).unwrap() ;
        self.canWrite = true ;
    }
}
