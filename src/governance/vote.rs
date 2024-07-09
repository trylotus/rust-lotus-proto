// This file is generated by rust-protobuf 3.5.0. Do not edit
// .proto file is parsed by protoc --rust_out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `lotus/governance/vote.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_0;

// @@protoc_insertion_point(message:lotus.governance.Vote)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Vote {
    // message fields
    // @@protoc_insertion_point(field:lotus.governance.Vote.timestamp)
    pub timestamp: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    // @@protoc_insertion_point(field:lotus.governance.Vote.voter_address)
    pub voter_address: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:lotus.governance.Vote.proposal_id)
    pub proposal_id: ::std::string::String,
    // @@protoc_insertion_point(field:lotus.governance.Vote.vote_option)
    pub vote_option: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:lotus.governance.Vote.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Vote {
    fn default() -> &'a Vote {
        <Vote as ::protobuf::Message>::default_instance()
    }
}

impl Vote {
    pub fn new() -> Vote {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "timestamp",
            |m: &Vote| { &m.timestamp },
            |m: &mut Vote| { &mut m.timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "voter_address",
            |m: &Vote| { &m.voter_address },
            |m: &mut Vote| { &mut m.voter_address },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "proposal_id",
            |m: &Vote| { &m.proposal_id },
            |m: &mut Vote| { &mut m.proposal_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "vote_option",
            |m: &Vote| { &m.vote_option },
            |m: &mut Vote| { &mut m.vote_option },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Vote>(
            "Vote",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Vote {
    const NAME: &'static str = "Vote";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.timestamp)?;
                },
                18 => {
                    self.voter_address = is.read_bytes()?;
                },
                26 => {
                    self.proposal_id = is.read_string()?;
                },
                34 => {
                    self.vote_option = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.voter_address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.voter_address);
        }
        if !self.proposal_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.proposal_id);
        }
        if !self.vote_option.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.vote_option);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.timestamp.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.voter_address.is_empty() {
            os.write_bytes(2, &self.voter_address)?;
        }
        if !self.proposal_id.is_empty() {
            os.write_string(3, &self.proposal_id)?;
        }
        if !self.vote_option.is_empty() {
            os.write_string(4, &self.vote_option)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Vote {
        Vote::new()
    }

    fn clear(&mut self) {
        self.timestamp.clear();
        self.voter_address.clear();
        self.proposal_id.clear();
        self.vote_option.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Vote {
        static instance: Vote = Vote {
            timestamp: ::protobuf::MessageField::none(),
            voter_address: ::std::vec::Vec::new(),
            proposal_id: ::std::string::String::new(),
            vote_option: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Vote {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Vote").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Vote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Vote {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1blotus/governance/vote.proto\x12\x10lotus.governance\x1a\x1fgoogle/\
    protobuf/timestamp.proto\x1a\x11lotus/lotus.proto\"\xaf\x01\n\x04Vote\
    \x128\n\ttimestamp\x18\x01\x20\x01(\x0b2\x1a.google.protobuf.TimestampR\
    \ttimestamp\x12+\n\rvoter_address\x18\x02\x20\x01(\x0cR\x0cvoterAddressB\
    \x06\xd2\xab0\x02\x08\x01\x12\x1f\n\x0bproposal_id\x18\x03\x20\x01(\tR\n\
    proposalId\x12\x1f\n\x0bvote_option\x18\x04\x20\x01(\tR\nvoteOptionB/Z-g\
    ithub.com/trylotus/go-lotus-proto/governanceJ\xdb\x02\n\x06\x12\x04\0\0\
    \x0e\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\0\
    \x19\n\t\n\x02\x03\0\x12\x03\x04\0)\n\t\n\x02\x03\x01\x12\x03\x05\0\x1b\
    \n\x08\n\x01\x08\x12\x03\x07\0D\n\t\n\x02\x08\x0b\x12\x03\x07\0D\n\n\n\
    \x02\x04\0\x12\x04\t\0\x0e\x01\n\n\n\x03\x04\0\x01\x12\x03\t\x08\x0c\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\n\x02*\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\n\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\x1c%\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\n()\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0b\x02C\
    \n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0b\x02\x07\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x0b\x08\x15\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0b\
    \x18\x19\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03\x0b\x1aB\n\x10\n\t\x04\0\
    \x02\x01\x08\xba\x85\x06\x01\x12\x03\x0b\x1bA\n\x0b\n\x04\x04\0\x02\x02\
    \x12\x03\x0c\x02\x19\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0c\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0c\t\x14\n\x0c\n\x05\x04\0\x02\x02\
    \x03\x12\x03\x0c\x17\x18\n\x0b\n\x04\x04\0\x02\x03\x12\x03\r\x02\x19\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\x03\
    \x01\x12\x03\r\t\x14\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\r\x17\x18b\
    \x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(::protobuf::well_known_types::timestamp::file_descriptor().clone());
            deps.push(super::lotus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Vote::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
