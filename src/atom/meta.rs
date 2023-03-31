use super::{Header, Mp4File};
/**

meta
    hdlr
    dinf
        dref
    ipmc
    iloc
    ipro
        sinf
            frma
            imif
            schm
            schi
    iinf
    xml
    bxml
    pitm
    fiin
        paen
            fpar
            fecr
        segr
        gitn
        tsel

**/
use std::string::String;

#[derive(Debug, Clone)]
pub struct Meta {
    header: Header,
}

impl Meta {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }
}

#[derive(Debug, Clone)]
pub struct Xml {
    header: Header,
    xml: String,
}

impl Xml {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);
        let mut xml_bytes: Vec<u8> = Vec::new();
        for _ in 0..header.data_size {
            xml_bytes.push(f.read_u8().unwrap());
        }
        let xml: String = String::from_utf8(xml_bytes).unwrap();

        f.offset_inc(header.data_size);
        Ok(Self { header, xml })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn xml(&self) -> &String {
        &self.xml
    }
}

#[derive(Debug, Clone)]
pub struct Bxml {
    header: Header,
    data: Vec<u8>,
}

impl Bxml {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);
        let mut data: Vec<u8> = Vec::new();
        for _ in 0..header.data_size {
            data.push(f.read_u8().unwrap());
        }
        // let xml: String = String::from_utf8(xml_bytes).unwrap();

        f.offset_inc(header.data_size);
        Ok(Self { header, data })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
