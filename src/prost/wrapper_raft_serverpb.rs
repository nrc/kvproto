impl RaftMessage {
    pub fn new_() -> RaftMessage {
        ::std::default::Default::default()
    }
    pub fn clear_region_id(&mut self) {
        self.region_id = 0
    }
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }
    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }
    pub fn has_from_peer(&self) -> bool {
        self.from_peer.is_some()
    }
    pub fn clear_from_peer(&mut self) {
        self.from_peer = ::std::option::Option::None
    }
    pub fn set_from_peer(&mut self, v: super::metapb::Peer) {
        self.from_peer = ::std::option::Option::Some(v);;    }
    pub fn get_from_peer(&self) -> &super::metapb::Peer {
        self.from_peer
            .as_ref()
            .unwrap_or_else(|| <super::metapb::Peer as ::protobuf::Message>::default_instance())
    }
    pub fn mut_from_peer(&mut self) -> &mut super::metapb::Peer {
        if self.from_peer.is_none() {
            self.from_peer = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.from_peer.as_mut().unwrap()
    }
    pub fn take_from_peer(&mut self) -> super::metapb::Peer {
        self.from_peer
            .take()
            .unwrap_or_else(|| super::metapb::Peer::default())
    }
    pub fn has_to_peer(&self) -> bool {
        self.to_peer.is_some()
    }
    pub fn clear_to_peer(&mut self) {
        self.to_peer = ::std::option::Option::None
    }
    pub fn set_to_peer(&mut self, v: super::metapb::Peer) {
        self.to_peer = ::std::option::Option::Some(v);;    }
    pub fn get_to_peer(&self) -> &super::metapb::Peer {
        self.to_peer
            .as_ref()
            .unwrap_or_else(|| <super::metapb::Peer as ::protobuf::Message>::default_instance())
    }
    pub fn mut_to_peer(&mut self) -> &mut super::metapb::Peer {
        if self.to_peer.is_none() {
            self.to_peer = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.to_peer.as_mut().unwrap()
    }
    pub fn take_to_peer(&mut self) -> super::metapb::Peer {
        self.to_peer
            .take()
            .unwrap_or_else(|| super::metapb::Peer::default())
    }
    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }
    pub fn clear_message(&mut self) {
        self.message = ::std::option::Option::None
    }
    pub fn set_message(&mut self, v: super::eraftpb::Message) {
        self.message = ::std::option::Option::Some(v);;    }
    pub fn get_message(&self) -> &super::eraftpb::Message {
        self.message
            .as_ref()
            .unwrap_or_else(|| <super::eraftpb::Message as ::protobuf::Message>::default_instance())
    }
    pub fn mut_message(&mut self) -> &mut super::eraftpb::Message {
        if self.message.is_none() {
            self.message = ::std::option::Option::Some(super::eraftpb::Message::default());
        }
        self.message.as_mut().unwrap()
    }
    pub fn take_message(&mut self) -> super::eraftpb::Message {
        self.message
            .take()
            .unwrap_or_else(|| super::eraftpb::Message::default())
    }
    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }
    pub fn clear_region_epoch(&mut self) {
        self.region_epoch = ::std::option::Option::None
    }
    pub fn set_region_epoch(&mut self, v: super::metapb::RegionEpoch) {
        self.region_epoch = ::std::option::Option::Some(v);;    }
    pub fn get_region_epoch(&self) -> &super::metapb::RegionEpoch {
        self.region_epoch.as_ref().unwrap_or_else(|| {
            <super::metapb::RegionEpoch as ::protobuf::Message>::default_instance()
        })
    }
    pub fn mut_region_epoch(&mut self) -> &mut super::metapb::RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch = ::std::option::Option::Some(super::metapb::RegionEpoch::default());
        }
        self.region_epoch.as_mut().unwrap()
    }
    pub fn take_region_epoch(&mut self) -> super::metapb::RegionEpoch {
        self.region_epoch
            .take()
            .unwrap_or_else(|| super::metapb::RegionEpoch::default())
    }
    pub fn clear_is_tombstone(&mut self) {
        self.is_tombstone = false
    }
    pub fn set_is_tombstone(&mut self, v: bool) {
        self.is_tombstone = v;
    }
    pub fn get_is_tombstone(&self) -> bool {
        self.is_tombstone
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
    pub fn has_merge_target(&self) -> bool {
        self.merge_target.is_some()
    }
    pub fn clear_merge_target(&mut self) {
        self.merge_target = ::std::option::Option::None
    }
    pub fn set_merge_target(&mut self, v: super::metapb::Region) {
        self.merge_target = ::std::option::Option::Some(v);;    }
    pub fn get_merge_target(&self) -> &super::metapb::Region {
        self.merge_target
            .as_ref()
            .unwrap_or_else(|| <super::metapb::Region as ::protobuf::Message>::default_instance())
    }
    pub fn mut_merge_target(&mut self) -> &mut super::metapb::Region {
        if self.merge_target.is_none() {
            self.merge_target = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.merge_target.as_mut().unwrap()
    }
    pub fn take_merge_target(&mut self) -> super::metapb::Region {
        self.merge_target
            .take()
            .unwrap_or_else(|| super::metapb::Region::default())
    }
}
impl ::protobuf::Clear for RaftMessage {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RaftMessage {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static RaftMessage {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl RaftTruncatedState {
    pub fn new_() -> RaftTruncatedState {
        ::std::default::Default::default()
    }
    pub fn clear_index(&mut self) {
        self.index = 0
    }
    pub fn set_index(&mut self, v: u64) {
        self.index = v;
    }
    pub fn get_index(&self) -> u64 {
        self.index
    }
    pub fn clear_term(&mut self) {
        self.term = 0
    }
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }
    pub fn get_term(&self) -> u64 {
        self.term
    }
}
impl ::protobuf::Clear for RaftTruncatedState {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RaftTruncatedState {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static RaftTruncatedState {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl SnapshotCfFile {
    pub fn new_() -> SnapshotCfFile {
        ::std::default::Default::default()
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
    pub fn clear_size(&mut self) {
        self.size = 0
    }
    pub fn set_size(&mut self, v: u64) {
        self.size = v;
    }
    pub fn get_size(&self) -> u64 {
        self.size
    }
    pub fn clear_checksum(&mut self) {
        self.checksum = 0
    }
    pub fn set_checksum(&mut self, v: u32) {
        self.checksum = v;
    }
    pub fn get_checksum(&self) -> u32 {
        self.checksum
    }
}
impl ::protobuf::Clear for SnapshotCfFile {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SnapshotCfFile {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static SnapshotCfFile {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl SnapshotMeta {
    pub fn new_() -> SnapshotMeta {
        ::std::default::Default::default()
    }
    pub fn clear_cf_files(&mut self) {
        self.cf_files.clear();
    }
    pub fn set_cf_files(&mut self, v: ::std::vec::Vec<SnapshotCfFile>) {
        self.cf_files = v;
    }
    pub fn get_cf_files(&self) -> &::std::vec::Vec<SnapshotCfFile> {
        &self.cf_files
    }
    pub fn mut_cf_files(&mut self) -> &mut ::std::vec::Vec<SnapshotCfFile> {
        &mut self.cf_files
    }
    pub fn take_cf_files(&mut self) -> ::std::vec::Vec<SnapshotCfFile> {
        ::std::mem::replace(&mut self.cf_files, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for SnapshotMeta {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SnapshotMeta {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static SnapshotMeta {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl SnapshotChunk {
    pub fn new_() -> SnapshotChunk {
        ::std::default::Default::default()
    }
    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }
    pub fn clear_message(&mut self) {
        self.message = ::std::option::Option::None
    }
    pub fn set_message(&mut self, v: RaftMessage) {
        self.message = ::std::option::Option::Some(v);;    }
    pub fn get_message(&self) -> &RaftMessage {
        self.message
            .as_ref()
            .unwrap_or_else(|| <RaftMessage as ::protobuf::Message>::default_instance())
    }
    pub fn mut_message(&mut self) -> &mut RaftMessage {
        if self.message.is_none() {
            self.message = ::std::option::Option::Some(RaftMessage::default());
        }
        self.message.as_mut().unwrap()
    }
    pub fn take_message(&mut self) -> RaftMessage {
        self.message
            .take()
            .unwrap_or_else(|| RaftMessage::default())
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
    pub fn set_data(&mut self, v: Vec<u8>) {
        self.data = v;
    }
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn mut_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    pub fn take_data(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for SnapshotChunk {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SnapshotChunk {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static SnapshotChunk {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl Done {
    pub fn new_() -> Done {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for Done {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Done {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static Done {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl KeyValue {
    pub fn new_() -> KeyValue {
        ::std::default::Default::default()
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for KeyValue {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for KeyValue {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static KeyValue {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl RaftSnapshotData {
    pub fn new_() -> RaftSnapshotData {
        ::std::default::Default::default()
    }
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }
    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None
    }
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::std::option::Option::Some(v);;    }
    pub fn get_region(&self) -> &super::metapb::Region {
        self.region
            .as_ref()
            .unwrap_or_else(|| <super::metapb::Region as ::protobuf::Message>::default_instance())
    }
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.region.as_mut().unwrap()
    }
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region
            .take()
            .unwrap_or_else(|| super::metapb::Region::default())
    }
    pub fn clear_file_size(&mut self) {
        self.file_size = 0
    }
    pub fn set_file_size(&mut self, v: u64) {
        self.file_size = v;
    }
    pub fn get_file_size(&self) -> u64 {
        self.file_size
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
    pub fn set_data(&mut self, v: ::std::vec::Vec<KeyValue>) {
        self.data = v;
    }
    pub fn get_data(&self) -> &::std::vec::Vec<KeyValue> {
        &self.data
    }
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<KeyValue> {
        &mut self.data
    }
    pub fn take_data(&mut self) -> ::std::vec::Vec<KeyValue> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
    pub fn get_version(&self) -> u64 {
        self.version
    }
    pub fn has_meta(&self) -> bool {
        self.meta.is_some()
    }
    pub fn clear_meta(&mut self) {
        self.meta = ::std::option::Option::None
    }
    pub fn set_meta(&mut self, v: SnapshotMeta) {
        self.meta = ::std::option::Option::Some(v);;    }
    pub fn get_meta(&self) -> &SnapshotMeta {
        self.meta
            .as_ref()
            .unwrap_or_else(|| <SnapshotMeta as ::protobuf::Message>::default_instance())
    }
    pub fn mut_meta(&mut self) -> &mut SnapshotMeta {
        if self.meta.is_none() {
            self.meta = ::std::option::Option::Some(SnapshotMeta::default());
        }
        self.meta.as_mut().unwrap()
    }
    pub fn take_meta(&mut self) -> SnapshotMeta {
        self.meta.take().unwrap_or_else(|| SnapshotMeta::default())
    }
}
impl ::protobuf::Clear for RaftSnapshotData {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RaftSnapshotData {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static RaftSnapshotData {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl StoreIdent {
    pub fn new_() -> StoreIdent {
        ::std::default::Default::default()
    }
    pub fn clear_cluster_id(&mut self) {
        self.cluster_id = 0
    }
    pub fn set_cluster_id(&mut self, v: u64) {
        self.cluster_id = v;
    }
    pub fn get_cluster_id(&self) -> u64 {
        self.cluster_id
    }
    pub fn clear_store_id(&mut self) {
        self.store_id = 0
    }
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = v;
    }
    pub fn get_store_id(&self) -> u64 {
        self.store_id
    }
}
impl ::protobuf::Clear for StoreIdent {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for StoreIdent {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static StoreIdent {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl RaftLocalState {
    pub fn new_() -> RaftLocalState {
        ::std::default::Default::default()
    }
    pub fn has_hard_state(&self) -> bool {
        self.hard_state.is_some()
    }
    pub fn clear_hard_state(&mut self) {
        self.hard_state = ::std::option::Option::None
    }
    pub fn set_hard_state(&mut self, v: super::eraftpb::HardState) {
        self.hard_state = ::std::option::Option::Some(v);;    }
    pub fn get_hard_state(&self) -> &super::eraftpb::HardState {
        self.hard_state.as_ref().unwrap_or_else(|| {
            <super::eraftpb::HardState as ::protobuf::Message>::default_instance()
        })
    }
    pub fn mut_hard_state(&mut self) -> &mut super::eraftpb::HardState {
        if self.hard_state.is_none() {
            self.hard_state = ::std::option::Option::Some(super::eraftpb::HardState::default());
        }
        self.hard_state.as_mut().unwrap()
    }
    pub fn take_hard_state(&mut self) -> super::eraftpb::HardState {
        self.hard_state
            .take()
            .unwrap_or_else(|| super::eraftpb::HardState::default())
    }
    pub fn clear_last_index(&mut self) {
        self.last_index = 0
    }
    pub fn set_last_index(&mut self, v: u64) {
        self.last_index = v;
    }
    pub fn get_last_index(&self) -> u64 {
        self.last_index
    }
}
impl ::protobuf::Clear for RaftLocalState {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RaftLocalState {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static RaftLocalState {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl RaftApplyState {
    pub fn new_() -> RaftApplyState {
        ::std::default::Default::default()
    }
    pub fn clear_applied_index(&mut self) {
        self.applied_index = 0
    }
    pub fn set_applied_index(&mut self, v: u64) {
        self.applied_index = v;
    }
    pub fn get_applied_index(&self) -> u64 {
        self.applied_index
    }
    pub fn has_truncated_state(&self) -> bool {
        self.truncated_state.is_some()
    }
    pub fn clear_truncated_state(&mut self) {
        self.truncated_state = ::std::option::Option::None
    }
    pub fn set_truncated_state(&mut self, v: RaftTruncatedState) {
        self.truncated_state = ::std::option::Option::Some(v);;    }
    pub fn get_truncated_state(&self) -> &RaftTruncatedState {
        self.truncated_state
            .as_ref()
            .unwrap_or_else(|| <RaftTruncatedState as ::protobuf::Message>::default_instance())
    }
    pub fn mut_truncated_state(&mut self) -> &mut RaftTruncatedState {
        if self.truncated_state.is_none() {
            self.truncated_state = ::std::option::Option::Some(RaftTruncatedState::default());
        }
        self.truncated_state.as_mut().unwrap()
    }
    pub fn take_truncated_state(&mut self) -> RaftTruncatedState {
        self.truncated_state
            .take()
            .unwrap_or_else(|| RaftTruncatedState::default())
    }
}
impl ::protobuf::Clear for RaftApplyState {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RaftApplyState {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static RaftApplyState {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl MergeState {
    pub fn new_() -> MergeState {
        ::std::default::Default::default()
    }
    pub fn clear_min_index(&mut self) {
        self.min_index = 0
    }
    pub fn set_min_index(&mut self, v: u64) {
        self.min_index = v;
    }
    pub fn get_min_index(&self) -> u64 {
        self.min_index
    }
    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }
    pub fn clear_target(&mut self) {
        self.target = ::std::option::Option::None
    }
    pub fn set_target(&mut self, v: super::metapb::Region) {
        self.target = ::std::option::Option::Some(v);;    }
    pub fn get_target(&self) -> &super::metapb::Region {
        self.target
            .as_ref()
            .unwrap_or_else(|| <super::metapb::Region as ::protobuf::Message>::default_instance())
    }
    pub fn mut_target(&mut self) -> &mut super::metapb::Region {
        if self.target.is_none() {
            self.target = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.target.as_mut().unwrap()
    }
    pub fn take_target(&mut self) -> super::metapb::Region {
        self.target
            .take()
            .unwrap_or_else(|| super::metapb::Region::default())
    }
    pub fn clear_commit(&mut self) {
        self.commit = 0
    }
    pub fn set_commit(&mut self, v: u64) {
        self.commit = v;
    }
    pub fn get_commit(&self) -> u64 {
        self.commit
    }
}
impl ::protobuf::Clear for MergeState {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for MergeState {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static MergeState {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
impl RegionLocalState {
    pub fn new_() -> RegionLocalState {
        ::std::default::Default::default()
    }
    pub fn clear_state(&mut self) {
        self.state = 0
    }
    pub fn set_state(&mut self, v: PeerState) {
        self.state = unsafe { ::std::mem::transmute::<PeerState, i32>(v) };
    }
    pub fn get_state(&self) -> PeerState {
        unsafe { ::std::mem::transmute::<i32, PeerState>(self.state) }
    }
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }
    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None
    }
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::std::option::Option::Some(v);;    }
    pub fn get_region(&self) -> &super::metapb::Region {
        self.region
            .as_ref()
            .unwrap_or_else(|| <super::metapb::Region as ::protobuf::Message>::default_instance())
    }
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.region.as_mut().unwrap()
    }
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region
            .take()
            .unwrap_or_else(|| super::metapb::Region::default())
    }
    pub fn has_merge_state(&self) -> bool {
        self.merge_state.is_some()
    }
    pub fn clear_merge_state(&mut self) {
        self.merge_state = ::std::option::Option::None
    }
    pub fn set_merge_state(&mut self, v: MergeState) {
        self.merge_state = ::std::option::Option::Some(v);;    }
    pub fn get_merge_state(&self) -> &MergeState {
        self.merge_state
            .as_ref()
            .unwrap_or_else(|| <MergeState as ::protobuf::Message>::default_instance())
    }
    pub fn mut_merge_state(&mut self) -> &mut MergeState {
        if self.merge_state.is_none() {
            self.merge_state = ::std::option::Option::Some(MergeState::default());
        }
        self.merge_state.as_mut().unwrap()
    }
    pub fn take_merge_state(&mut self) -> MergeState {
        self.merge_state
            .take()
            .unwrap_or_else(|| MergeState::default())
    }
}
impl ::protobuf::Clear for RegionLocalState {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RegionLocalState {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static RegionLocalState {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
}
