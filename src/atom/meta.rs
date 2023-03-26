use super::{Atom, Header, Kind, Mp4File};
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

#[allow(clippy::doc_markdown)]
/**
8.11.1 The Meta box
8.11.1.1 Definition

Box Type : `meta`
Container: File, Movie Box (‘moov’), Track Box (‘trak’), or Additional Metadata Container Box (‘meco’)
Mandatory: No
Quantity : Zero or one (in File, ‘moov’, and ‘trak’), One or more (in ‘meco’)

A meta box contains descriptive or annotative metadata. The 'meta' box is required to contain a ‘hdlr’
box indicating the structure or format of the ‘meta’ box contents.
That metadata is located either within a box within this box (e.g. an XML box),
or is located by the item identified by a primary item box.

All other contained boxes are specific to the format specified by the handler box.
The other boxes defined here may be defined as optional or mandatory for a given format.
If they are used, then they must take the form specified here. These optional boxes include
a data-information box, which documents other files in which metadata values (e.g. pictures) are placed, and a item location box, which documents where in those files each item is located (e.g. in the common case of multiple pictures stored in the same file). At most one meta box may occur at each of the file level, movie level, or track level, unless they are contained in an additional metadata container box (‘meco’).
If an Item Protection Box occurs, then some or all of the meta-data, including possibly the primary resource, may have been protected and be un-readable unless the protection system is taken into account.

8.11.1.2 Syntax
aligned(8) class MetaBox (handler_type) extends FullBox(‘meta’, version = 0, 0) {
    HandlerBox(handler_type) theHandler;
    PrimaryItemBox           primary_resource; // optional
    DataInformationBox       file_locations;   // optional
    ItemLocationBox          item_locations;   // optional
    ItemProtectionBox        protections;      // optional
    ItemInfoBox              item_infos;       // optional
    IPMPControlBox           IPMP_control;     // optional
    ItemReferenceBox         item_refs;        // optional
    ItemDataBox              item_data;        // optional
    Box   other_boxes[];
}

8.11.1.3 Semantics

The structure or format of the metadata is declared by the handler.
In the case that the primary data is identified by a primary item,
and that primary item has an item information entry with an item_type,
the handler type may be the same as the item_type.
**/
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
        Ok(Self {
            header,
            xml,
        })
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
        Ok(Self {
            header,
            data,
        })
    }
}
