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
        f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }
}

/**

8.11.2 XML Boxes
8.11.2.1 Definition

Box Type : `xml ` or `bxml`
Container: Meta box (`meta`)
Mandatory: No
Quantity : Zero or one

When the primary data is in XML format and it is desired that the XML be stored directly
in the meta-box, one of these forms may be used. The Binary XML Box may only be used
when there is a single well-defined binarization of the XML for that defined format as
identified by the handler.

Within an XML box the data is in UTF-8 format unless the data starts with
a byte-order-mark (BOM), which indicates that the data is in UTF-16 format.

8.11.2.2 Syntax

aligned(8) class `XMLBox` extends FullBox(‘xml ’, version = 0, 0) {
    string xml;
}

aligned(8) class `BinaryXMLBox` extends FullBox(‘bxml’, version = 0, 0) {
    unsigned int(8) data[]; // to end of box
}

**/

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

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }

    pub fn xml_ref(&self) -> &String {
        &self.xml
    }

    pub fn xml(&self) -> String {
        self.xml.clone()
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

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }

    pub fn data_ref(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}
