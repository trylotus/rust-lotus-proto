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

//! Generated file from `lotus/staking/staking.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_0;

// @@protoc_insertion_point(message:lotus.staking.Staking)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Staking {
    // message fields
    // @@protoc_insertion_point(field:lotus.staking.Staking.timestamp)
    pub timestamp: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    // @@protoc_insertion_point(field:lotus.staking.Staking.staker_address)
    pub staker_address: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:lotus.staking.Staking.amount)
    pub amount: ::std::string::String,
    // @@protoc_insertion_point(field:lotus.staking.Staking.staking_duration)
    pub staking_duration: u64,
    // @@protoc_insertion_point(field:lotus.staking.Staking.reward_amount)
    pub reward_amount: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:lotus.staking.Staking.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Staking {
    fn default() -> &'a Staking {
        <Staking as ::protobuf::Message>::default_instance()
    }
}

impl Staking {
    pub fn new() -> Staking {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "timestamp",
            |m: &Staking| { &m.timestamp },
            |m: &mut Staking| { &mut m.timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "staker_address",
            |m: &Staking| { &m.staker_address },
            |m: &mut Staking| { &mut m.staker_address },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "amount",
            |m: &Staking| { &m.amount },
            |m: &mut Staking| { &mut m.amount },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "staking_duration",
            |m: &Staking| { &m.staking_duration },
            |m: &mut Staking| { &mut m.staking_duration },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reward_amount",
            |m: &Staking| { &m.reward_amount },
            |m: &mut Staking| { &mut m.reward_amount },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Staking>(
            "Staking",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Staking {
    const NAME: &'static str = "Staking";

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
                    self.staker_address = is.read_bytes()?;
                },
                26 => {
                    self.amount = is.read_string()?;
                },
                32 => {
                    self.staking_duration = is.read_uint64()?;
                },
                42 => {
                    self.reward_amount = is.read_string()?;
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
        if !self.staker_address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.staker_address);
        }
        if !self.amount.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.amount);
        }
        if self.staking_duration != 0 {
            my_size += ::protobuf::rt::uint64_size(4, self.staking_duration);
        }
        if !self.reward_amount.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.reward_amount);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.timestamp.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.staker_address.is_empty() {
            os.write_bytes(2, &self.staker_address)?;
        }
        if !self.amount.is_empty() {
            os.write_string(3, &self.amount)?;
        }
        if self.staking_duration != 0 {
            os.write_uint64(4, self.staking_duration)?;
        }
        if !self.reward_amount.is_empty() {
            os.write_string(5, &self.reward_amount)?;
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

    fn new() -> Staking {
        Staking::new()
    }

    fn clear(&mut self) {
        self.timestamp.clear();
        self.staker_address.clear();
        self.amount.clear();
        self.staking_duration = 0;
        self.reward_amount.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Staking {
        static instance: Staking = Staking {
            timestamp: ::protobuf::MessageField::none(),
            staker_address: ::std::vec::Vec::new(),
            amount: ::std::string::String::new(),
            staking_duration: 0,
            reward_amount: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Staking {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Staking").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Staking {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Staking {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1blotus/staking/staking.proto\x12\rlotus.staking\x1a\x1fgoogle/proto\
    buf/timestamp.proto\x1a\x11lotus/lotus.proto\"\xe6\x01\n\x07Staking\x128\
    \n\ttimestamp\x18\x01\x20\x01(\x0b2\x1a.google.protobuf.TimestampR\ttime\
    stamp\x12-\n\x0estaker_address\x18\x02\x20\x01(\x0cR\rstakerAddressB\x06\
    \xd2\xab0\x02\x08\x01\x12\x1c\n\x06amount\x18\x03\x20\x01(\tR\x06amountB\
    \x04\xda\xab0\0\x12)\n\x10staking_duration\x18\x04\x20\x01(\x04R\x0fstak\
    ingDuration\x12)\n\rreward_amount\x18\x05\x20\x01(\tR\x0crewardAmountB\
    \x04\xda\xab0\0B,Z*github.com/trylotus/go-lotus-proto/stakingJ\xd0\x03\n\
    \x06\x12\x04\0\0\x0f\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x02\0\x16\n\t\n\x02\x03\0\x12\x03\x04\0)\n\t\n\x02\x03\x01\x12\
    \x03\x05\0\x1b\n\x08\n\x01\x08\x12\x03\x07\0A\n\t\n\x02\x08\x0b\x12\x03\
    \x07\0A\n\n\n\x02\x04\0\x12\x04\t\0\x0f\x01\n\n\n\x03\x04\0\x01\x12\x03\
    \t\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x02*\n\x0c\n\x05\x04\0\x02\
    \0\x06\x12\x03\n\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\x1c%\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\n()\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x0b\x02D\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0b\x02\x07\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x0b\x08\x16\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x0b\x19\x1a\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03\x0b\x1bC\n\x10\n\
    \t\x04\0\x02\x01\x08\xba\x85\x06\x01\x12\x03\x0b\x1cB\n\x0b\n\x04\x04\0\
    \x02\x02\x12\x03\x0c\x02+\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0c\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0c\t\x0f\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x03\x0c\x12\x13\n\x0c\n\x05\x04\0\x02\x02\x08\x12\x03\
    \x0c\x14*\n\x0f\n\x08\x04\0\x02\x02\x08\xbb\x85\x06\x12\x03\x0c\x15)\n\
    \x0b\n\x04\x04\0\x02\x03\x12\x03\r\x02\x1e\n\x0c\n\x05\x04\0\x02\x03\x05\
    \x12\x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\r\t\x19\n\x0c\n\
    \x05\x04\0\x02\x03\x03\x12\x03\r\x1c\x1d\n\x0b\n\x04\x04\0\x02\x04\x12\
    \x03\x0e\x022\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x0e\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x03\x0e\t\x16\n\x0c\n\x05\x04\0\x02\x04\x03\
    \x12\x03\x0e\x19\x1a\n\x0c\n\x05\x04\0\x02\x04\x08\x12\x03\x0e\x1b1\n\
    \x0f\n\x08\x04\0\x02\x04\x08\xbb\x85\x06\x12\x03\x0e\x1c0b\x06proto3\
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
            messages.push(Staking::generated_message_descriptor_data());
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
