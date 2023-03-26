use super::{Atom, Header, Kind, Mp4File};
/**

meco
    mere

**/
use std::string::String;

/**

8.11.7 Additional Metadata Container Box
8.11.7.1 Definition

Box Type : `meco`
Container: File, Movie Box (‘moov’), or Track Box (‘trak’)
Mandatory: No
Quantity : Zero or one

8.11.7.2 Syntax

aligned(8) class `AdditionalMetadataContainerBox` extends Box('meco') {

}
**/

#[derive(Debug, Clone)]
pub struct Meco {
    header: Header,
    children: Vec<Atom>,
}

impl Meco {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }
}

#[allow(clippy::doc_markdown)]
/**
8.11.8 Metabox Relation Box
8.11.8.1 Definition

Box Type : ‘mere’
Container: Additional Metadata Container Box (‘meco’)
Mandatory: No
Quantity : Zero or more

The metabox relation box indicates a relation between two meta boxes at the same level,
i.e., the top level of the file, the Movie Box, or Track Box. The relation between
two meta boxes is unspecified if there is no metabox relation box for those meta boxes.
Meta boxes are referenced by specifying their handler types.

8.11.8.2 Syntax

aligned(8) class `MetaboxRelationBox` extends FullBox('mere', version=0, 0) {
    unsigned int(32) first_metabox_handler_type;
    unsigned int(32) second_metabox_handler_type;
    unsigned int(8)  metabox_relation;
}

8.11.8.3 Semantics

`first_metabox_handler_type` indicates the first meta box to be related.
`second_metabox_handler_type` indicates the second meta box to be related.
`metabox_relation` indicates the relation between the two meta boxes. The following values are defined:
    1 The relationship between the boxes is unknown (which is the default when this box is not present);
    2 the two boxes are semantically un-related (e.g., one is presentation, the other annotation);
    3 the two boxes are semantically related but complementary (e.g., two disjoint sets of meta-data expressed in two different meta-data systems);
    4 the two boxes are semantically related but overlap (e.g., two sets of meta-data neither of which is a subset of the other); neither is ‘preferred’ to the other;
    5 the two boxes are semantically related but the second is a proper subset or weaker version of the first; the first is preferred;
    6 the two boxes are semantically related and equivalent (e.g., two essentially identical sets of meta-data expressed in two different meta-data systems).

8.11.9 URL Forms for meta boxes

When a meta-box is used, then URLs may be used to refer to items in the meta-box,
either using an absolute URL, or using a relative URL. Absolute URLs may only be used
to refer to items in a file-level meta box.

    When interpreting data that is in the context of a meta-box (i.e. the file for a file-level meta-box,
    the presentation for a movie-level meta-box, or the track for a track-level meta-box),
    the items in the meta-box are treated as shadowing files in the same location as that from which
    the container file came. This shadowing means that a reference to another file in the same location
    as the container file may be resolved to an item within the container file itself. Items can be
    addressed within the container file by appending a fragment to the URL for the container file itself.
    That fragment starts with the “#” character and consists of either:

        a) item_ID=<n>, identifying the item by its ID (the ID may be 0 for the primary resource);
        b) item_name=<item_name>, when the item information box is used.

    If a fragment within the contained item must be addressed, then the initial “#” character of
    that fragment is replaced by “*”.

    Consider the following example: <http://a.com/d/v.qrv#item_name=tree.html*branch1>.
    We assume that v.qrv is a file with a meta-box at the file level. First, the client strips the
    fragment and fetches v.qrv from a.com using HTTP. It then inspects the top-level meta box and
    adds the items in it, logically, to its cache of the directory “d” on a.com. It then re-forms
    the URL as <http://a.com/d/tree.html#branch1>. Note that the fragment has been elevated to a
    full file name, and the first “*” has been transformed back into a “#”. The client then either
    finds an item named tree.html in the meta box, or fetches tree.html from a.com, and it then
    finds the anchor “branch1” within tree.html. If within that html, a file was referenced
    using a relative URL, e.g. “flower.gif”, then the client converts this to an absolute URL
    using the normal rules: <http://a.com/d/flower.gif> and again it checks to see if flower.gif is
    a named item (and hence shadowing a separate file of this name), and then if it is not, fetches flower.gif
    from a.com.

8.11.10 Static Metadata

This section defines the storage of static (un-timed) metadata in the ISO file format family.
Reader support for metadata in general is optional, and therefore it is also optional for
the formats defined here or elsewhere, unless made mandatory by a derived specification.

8.11.10.1 Simple textual

There is existing support for simple textual tags in the form of the user-data boxes;
currently only one is defined – the copyright notice. Other metadata is permitted using this simple form if:
    a) it uses a registered box-type or it uses the UUID escape (the latter is permitted today);
    b) it uses a registered tag, the equivalent MPEG-7 construct must be documented as part of the registration.

8.11.10.2 Other forms

When other forms of metadata are desired, then a ‘meta’ box as defined above may be
included at the appropriate level of the document. If the document is intended to
be primarily a metadata document per se, then the meta box is at file level.
If the metadata annotates an entire presentation, then the meta box is at the movie level;
an entire stream, at the track level.

8.11.10.3 MPEG-7 metadata

MPEG-7 metadata is stored in meta boxes to this specification.

    1) The handler-type is ‘mp7t’ for textual metadata in Unicode format;
    2) The handler-type is ‘mp7b’ for binary metadata compressed in the BIM format.
        In this case, the binary XML box contains the configuration information
        immediately followed by the binarized XML.
    3) When the format is textual, there is either another box in the metadata container ‘meta’,
        called ‘xml’, which contains the textual MPEG-7 document, or there is a primary item box
        identifying the item containing the MPEG-7 XML.
    4) When the format is binary, there is either another box in the metadata container ‘meta’,
        called ‘bxml’, which contains the binary MPEG-7 document, or a primary item box identifying
        the item containing the MPEG-7 binarized XML.
    5) If an MPEG-7 box is used at the file level, then the brand ‘mp71’ should be a member of
        the compatible-brands list in the file-type box.

**/

#[derive(Debug, Clone)]
pub struct Mere {
    header: Header,
    first_metabox_handler_type: u32,
    second_metabox_handler_type: u32,
    metabox_relation: u8,
}

impl Mere {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);
        let first_metabox_handler_type = f.read_u32().unwrap_or(0);
        let second_metabox_handler_type = f.read_u32().unwrap_or(0);
        let metabox_relation = f.read_u8().unwrap_or(1);

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            first_metabox_handler_type,
            second_metabox_handler_type,
            metabox_relation,
        })
    }
}
