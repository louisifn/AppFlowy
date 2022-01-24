// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `folder_info.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct FolderInfo {
    // message fields
    pub folder_id: ::std::string::String,
    pub text: ::std::string::String,
    pub rev_id: i64,
    pub base_rev_id: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FolderInfo {
    fn default() -> &'a FolderInfo {
        <FolderInfo as ::protobuf::Message>::default_instance()
    }
}

impl FolderInfo {
    pub fn new() -> FolderInfo {
        ::std::default::Default::default()
    }

    // string folder_id = 1;


    pub fn get_folder_id(&self) -> &str {
        &self.folder_id
    }
    pub fn clear_folder_id(&mut self) {
        self.folder_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_folder_id(&mut self, v: ::std::string::String) {
        self.folder_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_folder_id(&mut self) -> &mut ::std::string::String {
        &mut self.folder_id
    }

    // Take field
    pub fn take_folder_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.folder_id, ::std::string::String::new())
    }

    // string text = 2;


    pub fn get_text(&self) -> &str {
        &self.text
    }
    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        &mut self.text
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.text, ::std::string::String::new())
    }

    // int64 rev_id = 3;


    pub fn get_rev_id(&self) -> i64 {
        self.rev_id
    }
    pub fn clear_rev_id(&mut self) {
        self.rev_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_rev_id(&mut self, v: i64) {
        self.rev_id = v;
    }

    // int64 base_rev_id = 4;


    pub fn get_base_rev_id(&self) -> i64 {
        self.base_rev_id
    }
    pub fn clear_base_rev_id(&mut self) {
        self.base_rev_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_base_rev_id(&mut self, v: i64) {
        self.base_rev_id = v;
    }
}

impl ::protobuf::Message for FolderInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.folder_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.text)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.rev_id = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.base_rev_id = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.folder_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.folder_id);
        }
        if !self.text.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.text);
        }
        if self.rev_id != 0 {
            my_size += ::protobuf::rt::value_size(3, self.rev_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.base_rev_id != 0 {
            my_size += ::protobuf::rt::value_size(4, self.base_rev_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.folder_id.is_empty() {
            os.write_string(1, &self.folder_id)?;
        }
        if !self.text.is_empty() {
            os.write_string(2, &self.text)?;
        }
        if self.rev_id != 0 {
            os.write_int64(3, self.rev_id)?;
        }
        if self.base_rev_id != 0 {
            os.write_int64(4, self.base_rev_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> FolderInfo {
        FolderInfo::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "folder_id",
                |m: &FolderInfo| { &m.folder_id },
                |m: &mut FolderInfo| { &mut m.folder_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "text",
                |m: &FolderInfo| { &m.text },
                |m: &mut FolderInfo| { &mut m.text },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "rev_id",
                |m: &FolderInfo| { &m.rev_id },
                |m: &mut FolderInfo| { &mut m.rev_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "base_rev_id",
                |m: &FolderInfo| { &m.base_rev_id },
                |m: &mut FolderInfo| { &mut m.base_rev_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<FolderInfo>(
                "FolderInfo",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static FolderInfo {
        static instance: ::protobuf::rt::LazyV2<FolderInfo> = ::protobuf::rt::LazyV2::INIT;
        instance.get(FolderInfo::new)
    }
}

impl ::protobuf::Clear for FolderInfo {
    fn clear(&mut self) {
        self.folder_id.clear();
        self.text.clear();
        self.rev_id = 0;
        self.base_rev_id = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FolderInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FolderInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11folder_info.proto\"t\n\nFolderInfo\x12\x1b\n\tfolder_id\x18\x01\
    \x20\x01(\tR\x08folderId\x12\x12\n\x04text\x18\x02\x20\x01(\tR\x04text\
    \x12\x15\n\x06rev_id\x18\x03\x20\x01(\x03R\x05revId\x12\x1e\n\x0bbase_re\
    v_id\x18\x04\x20\x01(\x03R\tbaseRevIdJ\x86\x02\n\x06\x12\x04\0\0\x07\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x07\x01\n\
    \n\n\x03\x04\0\x01\x12\x03\x02\x08\x12\n\x0b\n\x04\x04\0\x02\0\x12\x03\
    \x03\x04\x19\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03\x03\x0b\x14\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\
    \x03\x17\x18\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x14\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x04\x0b\x0f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x12\x13\n\x0b\
    \n\x04\x04\0\x02\x02\x12\x03\x05\x04\x15\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x03\x05\x04\t\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\n\x10\n\x0c\
    \n\x05\x04\0\x02\x02\x03\x12\x03\x05\x13\x14\n\x0b\n\x04\x04\0\x02\x03\
    \x12\x03\x06\x04\x1a\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x06\x04\t\n\
    \x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x06\n\x15\n\x0c\n\x05\x04\0\x02\x03\
    \x03\x12\x03\x06\x18\x19b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
