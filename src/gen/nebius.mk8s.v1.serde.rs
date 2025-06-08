// @generated
impl serde::Serialize for AttachedFilesystemSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attach_mode != 0 {
            len += 1;
        }
        if !self.mount_tag.is_empty() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.AttachedFilesystemSpec", len)?;
        if self.attach_mode != 0 {
            let v = attached_filesystem_spec::AttachMode::try_from(self.attach_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.attach_mode)))?;
            struct_ser.serialize_field("attachMode", &v)?;
        }
        if !self.mount_tag.is_empty() {
            struct_ser.serialize_field("mountTag", &self.mount_tag)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                attached_filesystem_spec::Type::ExistingFilesystem(v) => {
                    struct_ser.serialize_field("existingFilesystem", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttachedFilesystemSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attach_mode",
            "attachMode",
            "mount_tag",
            "mountTag",
            "existing_filesystem",
            "existingFilesystem",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttachMode,
            MountTag,
            ExistingFilesystem,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attachMode" | "attach_mode" => Ok(GeneratedField::AttachMode),
                            "mountTag" | "mount_tag" => Ok(GeneratedField::MountTag),
                            "existingFilesystem" | "existing_filesystem" => Ok(GeneratedField::ExistingFilesystem),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttachedFilesystemSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.AttachedFilesystemSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttachedFilesystemSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attach_mode__ = None;
                let mut mount_tag__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttachMode => {
                            if attach_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachMode"));
                            }
                            attach_mode__ = Some(map_.next_value::<attached_filesystem_spec::AttachMode>()? as i32);
                        }
                        GeneratedField::MountTag => {
                            if mount_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mountTag"));
                            }
                            mount_tag__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExistingFilesystem => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("existingFilesystem"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(attached_filesystem_spec::Type::ExistingFilesystem)
;
                        }
                    }
                }
                Ok(AttachedFilesystemSpec {
                    attach_mode: attach_mode__.unwrap_or_default(),
                    mount_tag: mount_tag__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.AttachedFilesystemSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attached_filesystem_spec::AttachMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::ReadOnly => "READ_ONLY",
            Self::ReadWrite => "READ_WRITE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for attached_filesystem_spec::AttachMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "READ_ONLY",
            "READ_WRITE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attached_filesystem_spec::AttachMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(attached_filesystem_spec::AttachMode::Unspecified),
                    "READ_ONLY" => Ok(attached_filesystem_spec::AttachMode::ReadOnly),
                    "READ_WRITE" => Ok(attached_filesystem_spec::AttachMode::ReadWrite),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Cluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.Cluster", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Cluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Cluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.Cluster")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Cluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Cluster {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.Cluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.control_plane.is_some() {
            len += 1;
        }
        if self.kube_network.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ClusterSpec", len)?;
        if let Some(v) = self.control_plane.as_ref() {
            struct_ser.serialize_field("controlPlane", v)?;
        }
        if let Some(v) = self.kube_network.as_ref() {
            struct_ser.serialize_field("kubeNetwork", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "control_plane",
            "controlPlane",
            "kube_network",
            "kubeNetwork",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ControlPlane,
            KubeNetwork,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "controlPlane" | "control_plane" => Ok(GeneratedField::ControlPlane),
                            "kubeNetwork" | "kube_network" => Ok(GeneratedField::KubeNetwork),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ClusterSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut control_plane__ = None;
                let mut kube_network__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ControlPlane => {
                            if control_plane__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controlPlane"));
                            }
                            control_plane__ = map_.next_value()?;
                        }
                        GeneratedField::KubeNetwork => {
                            if kube_network__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kubeNetwork"));
                            }
                            kube_network__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClusterSpec {
                    control_plane: control_plane__,
                    kube_network: kube_network__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ClusterSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state != 0 {
            len += 1;
        }
        if self.control_plane.is_some() {
            len += 1;
        }
        if self.reconciling {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ClusterStatus", len)?;
        if self.state != 0 {
            let v = cluster_status::State::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.control_plane.as_ref() {
            struct_ser.serialize_field("controlPlane", v)?;
        }
        if self.reconciling {
            struct_ser.serialize_field("reconciling", &self.reconciling)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "control_plane",
            "controlPlane",
            "reconciling",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            ControlPlane,
            Reconciling,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "state" => Ok(GeneratedField::State),
                            "controlPlane" | "control_plane" => Ok(GeneratedField::ControlPlane),
                            "reconciling" => Ok(GeneratedField::Reconciling),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ClusterStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut control_plane__ = None;
                let mut reconciling__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<cluster_status::State>()? as i32);
                        }
                        GeneratedField::ControlPlane => {
                            if control_plane__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controlPlane"));
                            }
                            control_plane__ = map_.next_value()?;
                        }
                        GeneratedField::Reconciling => {
                            if reconciling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reconciling"));
                            }
                            reconciling__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClusterStatus {
                    state: state__.unwrap_or_default(),
                    control_plane: control_plane__,
                    reconciling: reconciling__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ClusterStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster_status::State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATE_UNSPECIFIED",
            Self::Provisioning => "PROVISIONING",
            Self::Running => "RUNNING",
            Self::Deleting => "DELETING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster_status::State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_UNSPECIFIED",
            "PROVISIONING",
            "RUNNING",
            "DELETING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster_status::State;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STATE_UNSPECIFIED" => Ok(cluster_status::State::Unspecified),
                    "PROVISIONING" => Ok(cluster_status::State::Provisioning),
                    "RUNNING" => Ok(cluster_status::State::Running),
                    "DELETING" => Ok(cluster_status::State::Deleting),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Condition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.last_transition_at.is_some() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        if self.severity != 0 {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.last_transition_error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.Condition", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if self.status != 0 {
            let v = condition::Status::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.last_transition_at.as_ref() {
            struct_ser.serialize_field("lastTransitionAt", v)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if self.severity != 0 {
            let v = condition::Severity::try_from(self.severity)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.severity)))?;
            struct_ser.serialize_field("severity", &v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.last_transition_error.as_ref() {
            struct_ser.serialize_field("lastTransitionError", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Condition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "status",
            "last_transition_at",
            "lastTransitionAt",
            "reason",
            "severity",
            "description",
            "last_transition_error",
            "lastTransitionError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Status,
            LastTransitionAt,
            Reason,
            Severity,
            Description,
            LastTransitionError,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "status" => Ok(GeneratedField::Status),
                            "lastTransitionAt" | "last_transition_at" => Ok(GeneratedField::LastTransitionAt),
                            "reason" => Ok(GeneratedField::Reason),
                            "severity" => Ok(GeneratedField::Severity),
                            "description" => Ok(GeneratedField::Description),
                            "lastTransitionError" | "last_transition_error" => Ok(GeneratedField::LastTransitionError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Condition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.Condition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Condition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut status__ = None;
                let mut last_transition_at__ = None;
                let mut reason__ = None;
                let mut severity__ = None;
                let mut description__ = None;
                let mut last_transition_error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<condition::Status>()? as i32);
                        }
                        GeneratedField::LastTransitionAt => {
                            if last_transition_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastTransitionAt"));
                            }
                            last_transition_at__ = map_.next_value()?;
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Severity => {
                            if severity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("severity"));
                            }
                            severity__ = Some(map_.next_value::<condition::Severity>()? as i32);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastTransitionError => {
                            if last_transition_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastTransitionError"));
                            }
                            last_transition_error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Condition {
                    r#type: r#type__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    last_transition_at: last_transition_at__,
                    reason: reason__.unwrap_or_default(),
                    severity: severity__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    last_transition_error: last_transition_error__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.Condition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for condition::Severity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "NONE",
            Self::Info => "INFO",
            Self::Error => "ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for condition::Severity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NONE",
            "INFO",
            "ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = condition::Severity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NONE" => Ok(condition::Severity::None),
                    "INFO" => Ok(condition::Severity::Info),
                    "ERROR" => Ok(condition::Severity::Error),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for condition::Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::True => "TRUE",
            Self::False => "FALSE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for condition::Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "TRUE",
            "FALSE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = condition::Status;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(condition::Status::Unknown),
                    "TRUE" => Ok(condition::Status::True),
                    "FALSE" => Ok(condition::Status::False),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for condition::TransitionError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reason.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.Condition.TransitionError", len)?;
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for condition::TransitionError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reason" => Ok(GeneratedField::Reason),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = condition::TransitionError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.Condition.TransitionError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<condition::TransitionError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(condition::TransitionError {
                    reason: reason__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.Condition.TransitionError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlPlaneEndpointsSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.public_endpoint.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ControlPlaneEndpointsSpec", len)?;
        if let Some(v) = self.public_endpoint.as_ref() {
            struct_ser.serialize_field("publicEndpoint", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlPlaneEndpointsSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_endpoint",
            "publicEndpoint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicEndpoint,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "publicEndpoint" | "public_endpoint" => Ok(GeneratedField::PublicEndpoint),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlPlaneEndpointsSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ControlPlaneEndpointsSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlPlaneEndpointsSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_endpoint__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PublicEndpoint => {
                            if public_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicEndpoint"));
                            }
                            public_endpoint__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ControlPlaneEndpointsSpec {
                    public_endpoint: public_endpoint__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ControlPlaneEndpointsSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlPlaneSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.subnet_id.is_empty() {
            len += 1;
        }
        if self.endpoints.is_some() {
            len += 1;
        }
        if self.etcd_cluster_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ControlPlaneSpec", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.subnet_id.is_empty() {
            struct_ser.serialize_field("subnetId", &self.subnet_id)?;
        }
        if let Some(v) = self.endpoints.as_ref() {
            struct_ser.serialize_field("endpoints", v)?;
        }
        if self.etcd_cluster_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("etcdClusterSize", ToString::to_string(&self.etcd_cluster_size).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlPlaneSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "subnet_id",
            "subnetId",
            "endpoints",
            "etcd_cluster_size",
            "etcdClusterSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            SubnetId,
            Endpoints,
            EtcdClusterSize,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "subnetId" | "subnet_id" => Ok(GeneratedField::SubnetId),
                            "endpoints" => Ok(GeneratedField::Endpoints),
                            "etcdClusterSize" | "etcd_cluster_size" => Ok(GeneratedField::EtcdClusterSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlPlaneSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ControlPlaneSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlPlaneSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut subnet_id__ = None;
                let mut endpoints__ = None;
                let mut etcd_cluster_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubnetId => {
                            if subnet_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subnetId"));
                            }
                            subnet_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Endpoints => {
                            if endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoints"));
                            }
                            endpoints__ = map_.next_value()?;
                        }
                        GeneratedField::EtcdClusterSize => {
                            if etcd_cluster_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("etcdClusterSize"));
                            }
                            etcd_cluster_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ControlPlaneSpec {
                    version: version__.unwrap_or_default(),
                    subnet_id: subnet_id__.unwrap_or_default(),
                    endpoints: endpoints__,
                    etcd_cluster_size: etcd_cluster_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ControlPlaneSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlPlaneStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if self.endpoints.is_some() {
            len += 1;
        }
        if self.etcd_cluster_size != 0 {
            len += 1;
        }
        if self.auth.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ControlPlaneStatus", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.endpoints.as_ref() {
            struct_ser.serialize_field("endpoints", v)?;
        }
        if self.etcd_cluster_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("etcdClusterSize", ToString::to_string(&self.etcd_cluster_size).as_str())?;
        }
        if let Some(v) = self.auth.as_ref() {
            struct_ser.serialize_field("auth", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlPlaneStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "endpoints",
            "etcd_cluster_size",
            "etcdClusterSize",
            "auth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Endpoints,
            EtcdClusterSize,
            Auth,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "endpoints" => Ok(GeneratedField::Endpoints),
                            "etcdClusterSize" | "etcd_cluster_size" => Ok(GeneratedField::EtcdClusterSize),
                            "auth" => Ok(GeneratedField::Auth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlPlaneStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ControlPlaneStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlPlaneStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut endpoints__ = None;
                let mut etcd_cluster_size__ = None;
                let mut auth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Endpoints => {
                            if endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoints"));
                            }
                            endpoints__ = map_.next_value()?;
                        }
                        GeneratedField::EtcdClusterSize => {
                            if etcd_cluster_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("etcdClusterSize"));
                            }
                            etcd_cluster_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Auth => {
                            if auth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auth"));
                            }
                            auth__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ControlPlaneStatus {
                    version: version__.unwrap_or_default(),
                    endpoints: endpoints__,
                    etcd_cluster_size: etcd_cluster_size__.unwrap_or_default(),
                    auth: auth__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ControlPlaneStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlPlaneStatusAuth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_ca_certificate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ControlPlaneStatusAuth", len)?;
        if !self.cluster_ca_certificate.is_empty() {
            struct_ser.serialize_field("clusterCaCertificate", &self.cluster_ca_certificate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlPlaneStatusAuth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_ca_certificate",
            "clusterCaCertificate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterCaCertificate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "clusterCaCertificate" | "cluster_ca_certificate" => Ok(GeneratedField::ClusterCaCertificate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlPlaneStatusAuth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ControlPlaneStatusAuth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlPlaneStatusAuth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_ca_certificate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClusterCaCertificate => {
                            if cluster_ca_certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterCaCertificate"));
                            }
                            cluster_ca_certificate__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ControlPlaneStatusAuth {
                    cluster_ca_certificate: cluster_ca_certificate__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ControlPlaneStatusAuth", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlPlaneStatusEndpoints {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.public_endpoint.is_empty() {
            len += 1;
        }
        if !self.private_endpoint.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ControlPlaneStatusEndpoints", len)?;
        if !self.public_endpoint.is_empty() {
            struct_ser.serialize_field("publicEndpoint", &self.public_endpoint)?;
        }
        if !self.private_endpoint.is_empty() {
            struct_ser.serialize_field("privateEndpoint", &self.private_endpoint)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlPlaneStatusEndpoints {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_endpoint",
            "publicEndpoint",
            "private_endpoint",
            "privateEndpoint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicEndpoint,
            PrivateEndpoint,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "publicEndpoint" | "public_endpoint" => Ok(GeneratedField::PublicEndpoint),
                            "privateEndpoint" | "private_endpoint" => Ok(GeneratedField::PrivateEndpoint),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlPlaneStatusEndpoints;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ControlPlaneStatusEndpoints")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlPlaneStatusEndpoints, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_endpoint__ = None;
                let mut private_endpoint__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PublicEndpoint => {
                            if public_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicEndpoint"));
                            }
                            public_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivateEndpoint => {
                            if private_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateEndpoint"));
                            }
                            private_endpoint__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ControlPlaneStatusEndpoints {
                    public_endpoint: public_endpoint__.unwrap_or_default(),
                    private_endpoint: private_endpoint__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ControlPlaneStatusEndpoints", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateClusterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.CreateClusterRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateClusterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "spec" => Ok(GeneratedField::Spec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.CreateClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateClusterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateClusterRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.CreateClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateNodeGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.CreateNodeGroupRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateNodeGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "spec" => Ok(GeneratedField::Spec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateNodeGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.CreateNodeGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateNodeGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateNodeGroupRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.CreateNodeGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteClusterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.DeleteClusterRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteClusterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.DeleteClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteClusterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteClusterRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.DeleteClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteNodeGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.DeleteNodeGroupRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteNodeGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteNodeGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.DeleteNodeGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteNodeGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteNodeGroupRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.DeleteNodeGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DiskSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block_size_bytes != 0 {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.DiskSpec", len)?;
        if self.block_size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("blockSizeBytes", ToString::to_string(&self.block_size_bytes).as_str())?;
        }
        if self.r#type != 0 {
            let v = disk_spec::DiskType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.size.as_ref() {
            match v {
                disk_spec::Size::SizeBytes(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("sizeBytes", ToString::to_string(&v).as_str())?;
                }
                disk_spec::Size::SizeKibibytes(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("sizeKibibytes", ToString::to_string(&v).as_str())?;
                }
                disk_spec::Size::SizeMebibytes(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("sizeMebibytes", ToString::to_string(&v).as_str())?;
                }
                disk_spec::Size::SizeGibibytes(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("sizeGibibytes", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiskSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_size_bytes",
            "blockSizeBytes",
            "type",
            "size_bytes",
            "sizeBytes",
            "size_kibibytes",
            "sizeKibibytes",
            "size_mebibytes",
            "sizeMebibytes",
            "size_gibibytes",
            "sizeGibibytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockSizeBytes,
            Type,
            SizeBytes,
            SizeKibibytes,
            SizeMebibytes,
            SizeGibibytes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockSizeBytes" | "block_size_bytes" => Ok(GeneratedField::BlockSizeBytes),
                            "type" => Ok(GeneratedField::Type),
                            "sizeBytes" | "size_bytes" => Ok(GeneratedField::SizeBytes),
                            "sizeKibibytes" | "size_kibibytes" => Ok(GeneratedField::SizeKibibytes),
                            "sizeMebibytes" | "size_mebibytes" => Ok(GeneratedField::SizeMebibytes),
                            "sizeGibibytes" | "size_gibibytes" => Ok(GeneratedField::SizeGibibytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiskSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.DiskSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DiskSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_size_bytes__ = None;
                let mut r#type__ = None;
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BlockSizeBytes => {
                            if block_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockSizeBytes"));
                            }
                            block_size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<disk_spec::DiskType>()? as i32);
                        }
                        GeneratedField::SizeBytes => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sizeBytes"));
                            }
                            size__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| disk_spec::Size::SizeBytes(x.0));
                        }
                        GeneratedField::SizeKibibytes => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sizeKibibytes"));
                            }
                            size__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| disk_spec::Size::SizeKibibytes(x.0));
                        }
                        GeneratedField::SizeMebibytes => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sizeMebibytes"));
                            }
                            size__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| disk_spec::Size::SizeMebibytes(x.0));
                        }
                        GeneratedField::SizeGibibytes => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sizeGibibytes"));
                            }
                            size__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| disk_spec::Size::SizeGibibytes(x.0));
                        }
                    }
                }
                Ok(DiskSpec {
                    block_size_bytes: block_size_bytes__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    size: size__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.DiskSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for disk_spec::DiskType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::NetworkSsd => "NETWORK_SSD",
            Self::NetworkHdd => "NETWORK_HDD",
            Self::NetworkSsdIoM3 => "NETWORK_SSD_IO_M3",
            Self::NetworkSsdNonReplicated => "NETWORK_SSD_NON_REPLICATED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for disk_spec::DiskType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "NETWORK_SSD",
            "NETWORK_HDD",
            "NETWORK_SSD_IO_M3",
            "NETWORK_SSD_NON_REPLICATED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = disk_spec::DiskType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(disk_spec::DiskType::Unspecified),
                    "NETWORK_SSD" => Ok(disk_spec::DiskType::NetworkSsd),
                    "NETWORK_HDD" => Ok(disk_spec::DiskType::NetworkHdd),
                    "NETWORK_SSD_IO_M3" => Ok(disk_spec::DiskType::NetworkSsdIoM3),
                    "NETWORK_SSD_NON_REPLICATED" => Ok(disk_spec::DiskType::NetworkSsdNonReplicated),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExistingFilesystem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ExistingFilesystem", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExistingFilesystem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExistingFilesystem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ExistingFilesystem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExistingFilesystem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExistingFilesystem {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ExistingFilesystem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetClusterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.resource_version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.GetClusterRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.resource_version.is_empty() {
            struct_ser.serialize_field("resourceVersion", &self.resource_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetClusterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "resource_version",
            "resourceVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ResourceVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "resourceVersion" | "resource_version" => Ok(GeneratedField::ResourceVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.GetClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetClusterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut resource_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceVersion => {
                            if resource_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceVersion"));
                            }
                            resource_version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetClusterRequest {
                    id: id__.unwrap_or_default(),
                    resource_version: resource_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.GetClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetNodeGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.resource_version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.GetNodeGroupRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.resource_version.is_empty() {
            struct_ser.serialize_field("resourceVersion", &self.resource_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetNodeGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "resource_version",
            "resourceVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ResourceVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "resourceVersion" | "resource_version" => Ok(GeneratedField::ResourceVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetNodeGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.GetNodeGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetNodeGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut resource_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceVersion => {
                            if resource_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceVersion"));
                            }
                            resource_version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetNodeGroupRequest {
                    id: id__.unwrap_or_default(),
                    resource_version: resource_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.GetNodeGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GpuClusterSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.GpuClusterSpec", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GpuClusterSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GpuClusterSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.GpuClusterSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GpuClusterSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GpuClusterSpec {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.GpuClusterSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GpuSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.drivers_preset.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.GpuSettings", len)?;
        if !self.drivers_preset.is_empty() {
            struct_ser.serialize_field("driversPreset", &self.drivers_preset)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GpuSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "drivers_preset",
            "driversPreset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DriversPreset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "driversPreset" | "drivers_preset" => Ok(GeneratedField::DriversPreset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GpuSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.GpuSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GpuSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut drivers_preset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DriversPreset => {
                            if drivers_preset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("driversPreset"));
                            }
                            drivers_preset__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GpuSettings {
                    drivers_preset: drivers_preset__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.GpuSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KubeNetworkSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service_cidrs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.KubeNetworkSpec", len)?;
        if !self.service_cidrs.is_empty() {
            struct_ser.serialize_field("serviceCidrs", &self.service_cidrs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KubeNetworkSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_cidrs",
            "serviceCidrs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceCidrs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "serviceCidrs" | "service_cidrs" => Ok(GeneratedField::ServiceCidrs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KubeNetworkSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.KubeNetworkSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KubeNetworkSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_cidrs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServiceCidrs => {
                            if service_cidrs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceCidrs"));
                            }
                            service_cidrs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(KubeNetworkSpec {
                    service_cidrs: service_cidrs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.KubeNetworkSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListClustersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent_id.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ListClustersRequest", len)?;
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", &self.parent_id)?;
        }
        if self.page_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageSize", ToString::to_string(&self.page_size).as_str())?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListClustersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent_id",
            "parentId",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParentId,
            PageSize,
            PageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListClustersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ListClustersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListClustersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListClustersRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ListClustersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListClustersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ListClustersResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListClustersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            NextPageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "items" => Ok(GeneratedField::Items),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListClustersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ListClustersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListClustersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListClustersResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ListClustersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListNodeGroupsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent_id.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ListNodeGroupsRequest", len)?;
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", &self.parent_id)?;
        }
        if self.page_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageSize", ToString::to_string(&self.page_size).as_str())?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListNodeGroupsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent_id",
            "parentId",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParentId,
            PageSize,
            PageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListNodeGroupsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ListNodeGroupsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListNodeGroupsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListNodeGroupsRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ListNodeGroupsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListNodeGroupsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ListNodeGroupsResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListNodeGroupsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            NextPageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "items" => Ok(GeneratedField::Items),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListNodeGroupsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ListNodeGroupsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListNodeGroupsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListNodeGroupsResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ListNodeGroupsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkInterfaceTemplate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.public_ip_address.is_some() {
            len += 1;
        }
        if !self.subnet_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.NetworkInterfaceTemplate", len)?;
        if let Some(v) = self.public_ip_address.as_ref() {
            struct_ser.serialize_field("publicIpAddress", v)?;
        }
        if !self.subnet_id.is_empty() {
            struct_ser.serialize_field("subnetId", &self.subnet_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NetworkInterfaceTemplate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_ip_address",
            "publicIpAddress",
            "subnet_id",
            "subnetId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicIpAddress,
            SubnetId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "publicIpAddress" | "public_ip_address" => Ok(GeneratedField::PublicIpAddress),
                            "subnetId" | "subnet_id" => Ok(GeneratedField::SubnetId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetworkInterfaceTemplate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.NetworkInterfaceTemplate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NetworkInterfaceTemplate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_ip_address__ = None;
                let mut subnet_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PublicIpAddress => {
                            if public_ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicIpAddress"));
                            }
                            public_ip_address__ = map_.next_value()?;
                        }
                        GeneratedField::SubnetId => {
                            if subnet_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subnetId"));
                            }
                            subnet_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NetworkInterfaceTemplate {
                    public_ip_address: public_ip_address__,
                    subnet_id: subnet_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.NetworkInterfaceTemplate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeGroup {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.NodeGroup", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.NodeGroup")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NodeGroup {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.NodeGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeGroupAutoscalingSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_node_count != 0 {
            len += 1;
        }
        if self.max_node_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.NodeGroupAutoscalingSpec", len)?;
        if self.min_node_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("minNodeCount", ToString::to_string(&self.min_node_count).as_str())?;
        }
        if self.max_node_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxNodeCount", ToString::to_string(&self.max_node_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeGroupAutoscalingSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_node_count",
            "minNodeCount",
            "max_node_count",
            "maxNodeCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinNodeCount,
            MaxNodeCount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "minNodeCount" | "min_node_count" => Ok(GeneratedField::MinNodeCount),
                            "maxNodeCount" | "max_node_count" => Ok(GeneratedField::MaxNodeCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeGroupAutoscalingSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.NodeGroupAutoscalingSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeGroupAutoscalingSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_node_count__ = None;
                let mut max_node_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinNodeCount => {
                            if min_node_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minNodeCount"));
                            }
                            min_node_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxNodeCount => {
                            if max_node_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxNodeCount"));
                            }
                            max_node_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(NodeGroupAutoscalingSpec {
                    min_node_count: min_node_count__.unwrap_or_default(),
                    max_node_count: max_node_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.NodeGroupAutoscalingSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeGroupDeploymentStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_unavailable.is_some() {
            len += 1;
        }
        if self.max_surge.is_some() {
            len += 1;
        }
        if self.drain_timeout.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.NodeGroupDeploymentStrategy", len)?;
        if let Some(v) = self.max_unavailable.as_ref() {
            struct_ser.serialize_field("maxUnavailable", v)?;
        }
        if let Some(v) = self.max_surge.as_ref() {
            struct_ser.serialize_field("maxSurge", v)?;
        }
        if let Some(v) = self.drain_timeout.as_ref() {
            struct_ser.serialize_field("drainTimeout", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeGroupDeploymentStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_unavailable",
            "maxUnavailable",
            "max_surge",
            "maxSurge",
            "drain_timeout",
            "drainTimeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxUnavailable,
            MaxSurge,
            DrainTimeout,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxUnavailable" | "max_unavailable" => Ok(GeneratedField::MaxUnavailable),
                            "maxSurge" | "max_surge" => Ok(GeneratedField::MaxSurge),
                            "drainTimeout" | "drain_timeout" => Ok(GeneratedField::DrainTimeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeGroupDeploymentStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.NodeGroupDeploymentStrategy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeGroupDeploymentStrategy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_unavailable__ = None;
                let mut max_surge__ = None;
                let mut drain_timeout__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxUnavailable => {
                            if max_unavailable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUnavailable"));
                            }
                            max_unavailable__ = map_.next_value()?;
                        }
                        GeneratedField::MaxSurge => {
                            if max_surge__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSurge"));
                            }
                            max_surge__ = map_.next_value()?;
                        }
                        GeneratedField::DrainTimeout => {
                            if drain_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drainTimeout"));
                            }
                            drain_timeout__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NodeGroupDeploymentStrategy {
                    max_unavailable: max_unavailable__,
                    max_surge: max_surge__,
                    drain_timeout: drain_timeout__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.NodeGroupDeploymentStrategy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeGroupSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if self.template.is_some() {
            len += 1;
        }
        if self.strategy.is_some() {
            len += 1;
        }
        if self.size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.NodeGroupSpec", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.template.as_ref() {
            struct_ser.serialize_field("template", v)?;
        }
        if let Some(v) = self.strategy.as_ref() {
            struct_ser.serialize_field("strategy", v)?;
        }
        if let Some(v) = self.size.as_ref() {
            match v {
                node_group_spec::Size::FixedNodeCount(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("fixedNodeCount", ToString::to_string(&v).as_str())?;
                }
                node_group_spec::Size::Autoscaling(v) => {
                    struct_ser.serialize_field("autoscaling", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeGroupSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "template",
            "strategy",
            "fixed_node_count",
            "fixedNodeCount",
            "autoscaling",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Template,
            Strategy,
            FixedNodeCount,
            Autoscaling,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "template" => Ok(GeneratedField::Template),
                            "strategy" => Ok(GeneratedField::Strategy),
                            "fixedNodeCount" | "fixed_node_count" => Ok(GeneratedField::FixedNodeCount),
                            "autoscaling" => Ok(GeneratedField::Autoscaling),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeGroupSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.NodeGroupSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeGroupSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut template__ = None;
                let mut strategy__ = None;
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Template => {
                            if template__.is_some() {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            template__ = map_.next_value()?;
                        }
                        GeneratedField::Strategy => {
                            if strategy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strategy"));
                            }
                            strategy__ = map_.next_value()?;
                        }
                        GeneratedField::FixedNodeCount => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedNodeCount"));
                            }
                            size__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| node_group_spec::Size::FixedNodeCount(x.0));
                        }
                        GeneratedField::Autoscaling => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoscaling"));
                            }
                            size__ = map_.next_value::<::std::option::Option<_>>()?.map(node_group_spec::Size::Autoscaling)
;
                        }
                    }
                }
                Ok(NodeGroupSpec {
                    version: version__.unwrap_or_default(),
                    template: template__,
                    strategy: strategy__,
                    size: size__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.NodeGroupSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeGroupStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state != 0 {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if self.target_node_count != 0 {
            len += 1;
        }
        if self.node_count != 0 {
            len += 1;
        }
        if self.outdated_node_count != 0 {
            len += 1;
        }
        if self.ready_node_count != 0 {
            len += 1;
        }
        if !self.conditions.is_empty() {
            len += 1;
        }
        if self.reconciling {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.NodeGroupStatus", len)?;
        if self.state != 0 {
            let v = node_group_status::State::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.target_node_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("targetNodeCount", ToString::to_string(&self.target_node_count).as_str())?;
        }
        if self.node_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nodeCount", ToString::to_string(&self.node_count).as_str())?;
        }
        if self.outdated_node_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("outdatedNodeCount", ToString::to_string(&self.outdated_node_count).as_str())?;
        }
        if self.ready_node_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("readyNodeCount", ToString::to_string(&self.ready_node_count).as_str())?;
        }
        if !self.conditions.is_empty() {
            struct_ser.serialize_field("conditions", &self.conditions)?;
        }
        if self.reconciling {
            struct_ser.serialize_field("reconciling", &self.reconciling)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeGroupStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "version",
            "target_node_count",
            "targetNodeCount",
            "node_count",
            "nodeCount",
            "outdated_node_count",
            "outdatedNodeCount",
            "ready_node_count",
            "readyNodeCount",
            "conditions",
            "reconciling",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            Version,
            TargetNodeCount,
            NodeCount,
            OutdatedNodeCount,
            ReadyNodeCount,
            Conditions,
            Reconciling,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "state" => Ok(GeneratedField::State),
                            "version" => Ok(GeneratedField::Version),
                            "targetNodeCount" | "target_node_count" => Ok(GeneratedField::TargetNodeCount),
                            "nodeCount" | "node_count" => Ok(GeneratedField::NodeCount),
                            "outdatedNodeCount" | "outdated_node_count" => Ok(GeneratedField::OutdatedNodeCount),
                            "readyNodeCount" | "ready_node_count" => Ok(GeneratedField::ReadyNodeCount),
                            "conditions" => Ok(GeneratedField::Conditions),
                            "reconciling" => Ok(GeneratedField::Reconciling),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeGroupStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.NodeGroupStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeGroupStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut version__ = None;
                let mut target_node_count__ = None;
                let mut node_count__ = None;
                let mut outdated_node_count__ = None;
                let mut ready_node_count__ = None;
                let mut conditions__ = None;
                let mut reconciling__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<node_group_status::State>()? as i32);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetNodeCount => {
                            if target_node_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetNodeCount"));
                            }
                            target_node_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NodeCount => {
                            if node_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeCount"));
                            }
                            node_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OutdatedNodeCount => {
                            if outdated_node_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outdatedNodeCount"));
                            }
                            outdated_node_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ReadyNodeCount => {
                            if ready_node_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readyNodeCount"));
                            }
                            ready_node_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Conditions => {
                            if conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            conditions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reconciling => {
                            if reconciling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reconciling"));
                            }
                            reconciling__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NodeGroupStatus {
                    state: state__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    target_node_count: target_node_count__.unwrap_or_default(),
                    node_count: node_count__.unwrap_or_default(),
                    outdated_node_count: outdated_node_count__.unwrap_or_default(),
                    ready_node_count: ready_node_count__.unwrap_or_default(),
                    conditions: conditions__.unwrap_or_default(),
                    reconciling: reconciling__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.NodeGroupStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for node_group_status::State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATE_UNSPECIFIED",
            Self::Provisioning => "PROVISIONING",
            Self::Running => "RUNNING",
            Self::Deleting => "DELETING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for node_group_status::State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_UNSPECIFIED",
            "PROVISIONING",
            "RUNNING",
            "DELETING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = node_group_status::State;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STATE_UNSPECIFIED" => Ok(node_group_status::State::Unspecified),
                    "PROVISIONING" => Ok(node_group_status::State::Provisioning),
                    "RUNNING" => Ok(node_group_status::State::Running),
                    "DELETING" => Ok(node_group_status::State::Deleting),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for NodeMetadataTemplate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.labels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.NodeMetadataTemplate", len)?;
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeMetadataTemplate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "labels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Labels,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "labels" => Ok(GeneratedField::Labels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeMetadataTemplate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.NodeMetadataTemplate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeMetadataTemplate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut labels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(NodeMetadataTemplate {
                    labels: labels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.NodeMetadataTemplate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeTaint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if self.effect != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.NodeTaint", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if self.effect != 0 {
            let v = node_taint::Effect::try_from(self.effect)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.effect)))?;
            struct_ser.serialize_field("effect", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeTaint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
            "effect",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
            Effect,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            "effect" => Ok(GeneratedField::Effect),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeTaint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.NodeTaint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeTaint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                let mut effect__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Effect => {
                            if effect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effect"));
                            }
                            effect__ = Some(map_.next_value::<node_taint::Effect>()? as i32);
                        }
                    }
                }
                Ok(NodeTaint {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    effect: effect__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.NodeTaint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for node_taint::Effect {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EFFECT_UNSPECIFIED",
            Self::NoExecute => "NO_EXECUTE",
            Self::NoSchedule => "NO_SCHEDULE",
            Self::PreferNoSchedule => "PREFER_NO_SCHEDULE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for node_taint::Effect {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EFFECT_UNSPECIFIED",
            "NO_EXECUTE",
            "NO_SCHEDULE",
            "PREFER_NO_SCHEDULE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = node_taint::Effect;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "EFFECT_UNSPECIFIED" => Ok(node_taint::Effect::Unspecified),
                    "NO_EXECUTE" => Ok(node_taint::Effect::NoExecute),
                    "NO_SCHEDULE" => Ok(node_taint::Effect::NoSchedule),
                    "PREFER_NO_SCHEDULE" => Ok(node_taint::Effect::PreferNoSchedule),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for NodeTemplate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if !self.taints.is_empty() {
            len += 1;
        }
        if self.resources.is_some() {
            len += 1;
        }
        if self.boot_disk.is_some() {
            len += 1;
        }
        if self.gpu_settings.is_some() {
            len += 1;
        }
        if self.gpu_cluster.is_some() {
            len += 1;
        }
        if !self.network_interfaces.is_empty() {
            len += 1;
        }
        if !self.filesystems.is_empty() {
            len += 1;
        }
        if !self.cloud_init_user_data.is_empty() {
            len += 1;
        }
        if !self.service_account_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.NodeTemplate", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.taints.is_empty() {
            struct_ser.serialize_field("taints", &self.taints)?;
        }
        if let Some(v) = self.resources.as_ref() {
            struct_ser.serialize_field("resources", v)?;
        }
        if let Some(v) = self.boot_disk.as_ref() {
            struct_ser.serialize_field("bootDisk", v)?;
        }
        if let Some(v) = self.gpu_settings.as_ref() {
            struct_ser.serialize_field("gpuSettings", v)?;
        }
        if let Some(v) = self.gpu_cluster.as_ref() {
            struct_ser.serialize_field("gpuCluster", v)?;
        }
        if !self.network_interfaces.is_empty() {
            struct_ser.serialize_field("networkInterfaces", &self.network_interfaces)?;
        }
        if !self.filesystems.is_empty() {
            struct_ser.serialize_field("filesystems", &self.filesystems)?;
        }
        if !self.cloud_init_user_data.is_empty() {
            struct_ser.serialize_field("cloudInitUserData", &self.cloud_init_user_data)?;
        }
        if !self.service_account_id.is_empty() {
            struct_ser.serialize_field("serviceAccountId", &self.service_account_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeTemplate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "taints",
            "resources",
            "boot_disk",
            "bootDisk",
            "gpu_settings",
            "gpuSettings",
            "gpu_cluster",
            "gpuCluster",
            "network_interfaces",
            "networkInterfaces",
            "filesystems",
            "cloud_init_user_data",
            "cloudInitUserData",
            "service_account_id",
            "serviceAccountId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Taints,
            Resources,
            BootDisk,
            GpuSettings,
            GpuCluster,
            NetworkInterfaces,
            Filesystems,
            CloudInitUserData,
            ServiceAccountId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "taints" => Ok(GeneratedField::Taints),
                            "resources" => Ok(GeneratedField::Resources),
                            "bootDisk" | "boot_disk" => Ok(GeneratedField::BootDisk),
                            "gpuSettings" | "gpu_settings" => Ok(GeneratedField::GpuSettings),
                            "gpuCluster" | "gpu_cluster" => Ok(GeneratedField::GpuCluster),
                            "networkInterfaces" | "network_interfaces" => Ok(GeneratedField::NetworkInterfaces),
                            "filesystems" => Ok(GeneratedField::Filesystems),
                            "cloudInitUserData" | "cloud_init_user_data" => Ok(GeneratedField::CloudInitUserData),
                            "serviceAccountId" | "service_account_id" => Ok(GeneratedField::ServiceAccountId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeTemplate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.NodeTemplate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeTemplate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut taints__ = None;
                let mut resources__ = None;
                let mut boot_disk__ = None;
                let mut gpu_settings__ = None;
                let mut gpu_cluster__ = None;
                let mut network_interfaces__ = None;
                let mut filesystems__ = None;
                let mut cloud_init_user_data__ = None;
                let mut service_account_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Taints => {
                            if taints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taints"));
                            }
                            taints__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = map_.next_value()?;
                        }
                        GeneratedField::BootDisk => {
                            if boot_disk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bootDisk"));
                            }
                            boot_disk__ = map_.next_value()?;
                        }
                        GeneratedField::GpuSettings => {
                            if gpu_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gpuSettings"));
                            }
                            gpu_settings__ = map_.next_value()?;
                        }
                        GeneratedField::GpuCluster => {
                            if gpu_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gpuCluster"));
                            }
                            gpu_cluster__ = map_.next_value()?;
                        }
                        GeneratedField::NetworkInterfaces => {
                            if network_interfaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkInterfaces"));
                            }
                            network_interfaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filesystems => {
                            if filesystems__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filesystems"));
                            }
                            filesystems__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CloudInitUserData => {
                            if cloud_init_user_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cloudInitUserData"));
                            }
                            cloud_init_user_data__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceAccountId => {
                            if service_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceAccountId"));
                            }
                            service_account_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NodeTemplate {
                    metadata: metadata__,
                    taints: taints__.unwrap_or_default(),
                    resources: resources__,
                    boot_disk: boot_disk__,
                    gpu_settings: gpu_settings__,
                    gpu_cluster: gpu_cluster__,
                    network_interfaces: network_interfaces__.unwrap_or_default(),
                    filesystems: filesystems__.unwrap_or_default(),
                    cloud_init_user_data: cloud_init_user_data__.unwrap_or_default(),
                    service_account_id: service_account_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.NodeTemplate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PercentOrCount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.PercentOrCount", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                percent_or_count::Value::Percent(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("percent", ToString::to_string(&v).as_str())?;
                }
                percent_or_count::Value::Count(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("count", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PercentOrCount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "percent",
            "count",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Percent,
            Count,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "percent" => Ok(GeneratedField::Percent),
                            "count" => Ok(GeneratedField::Count),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PercentOrCount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.PercentOrCount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PercentOrCount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Percent => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("percent"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| percent_or_count::Value::Percent(x.0));
                        }
                        GeneratedField::Count => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| percent_or_count::Value::Count(x.0));
                        }
                    }
                }
                Ok(PercentOrCount {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.PercentOrCount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Problem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stage.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.Problem", len)?;
        if !self.stage.is_empty() {
            struct_ser.serialize_field("stage", &self.stage)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Problem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stage",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stage,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stage" => Ok(GeneratedField::Stage),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Problem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.Problem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Problem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stage__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Stage => {
                            if stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stage"));
                            }
                            stage__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Problem {
                    stage: stage__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.Problem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProgressData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.problems.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ProgressData", len)?;
        if !self.problems.is_empty() {
            struct_ser.serialize_field("problems", &self.problems)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProgressData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "problems",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Problems,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "problems" => Ok(GeneratedField::Problems),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProgressData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ProgressData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProgressData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut problems__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Problems => {
                            if problems__.is_some() {
                                return Err(serde::de::Error::duplicate_field("problems"));
                            }
                            problems__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProgressData {
                    problems: problems__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ProgressData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublicEndpointSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nebius.mk8s.v1.PublicEndpointSpec", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublicEndpointSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublicEndpointSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.PublicEndpointSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PublicEndpointSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PublicEndpointSpec {
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.PublicEndpointSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublicIpAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nebius.mk8s.v1.PublicIPAddress", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublicIpAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublicIpAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.PublicIPAddress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PublicIpAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PublicIpAddress {
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.PublicIPAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourcesSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.platform.is_empty() {
            len += 1;
        }
        if self.size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.ResourcesSpec", len)?;
        if !self.platform.is_empty() {
            struct_ser.serialize_field("platform", &self.platform)?;
        }
        if let Some(v) = self.size.as_ref() {
            match v {
                resources_spec::Size::Preset(v) => {
                    struct_ser.serialize_field("preset", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourcesSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "platform",
            "preset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Platform,
            Preset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "platform" => Ok(GeneratedField::Platform),
                            "preset" => Ok(GeneratedField::Preset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourcesSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.ResourcesSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourcesSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut platform__ = None;
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Platform => {
                            if platform__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platform"));
                            }
                            platform__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Preset => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preset"));
                            }
                            size__ = map_.next_value::<::std::option::Option<_>>()?.map(resources_spec::Size::Preset);
                        }
                    }
                }
                Ok(ResourcesSpec {
                    platform: platform__.unwrap_or_default(),
                    size: size__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.ResourcesSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateClusterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.UpdateClusterRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateClusterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "spec" => Ok(GeneratedField::Spec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.UpdateClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateClusterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateClusterRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.UpdateClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateNodeGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.UpdateNodeGroupRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateNodeGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "spec" => Ok(GeneratedField::Spec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateNodeGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.UpdateNodeGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateNodeGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateNodeGroupRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.UpdateNodeGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpgradeNodeGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.upgrade_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.mk8s.v1.UpgradeNodeGroupRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.upgrade_type.as_ref() {
            match v {
                upgrade_node_group_request::UpgradeType::LatestInfraVersion(v) => {
                    struct_ser.serialize_field("latestInfraVersion", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpgradeNodeGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "latest_infra_version",
            "latestInfraVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            LatestInfraVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "latestInfraVersion" | "latest_infra_version" => Ok(GeneratedField::LatestInfraVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpgradeNodeGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.mk8s.v1.UpgradeNodeGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpgradeNodeGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut upgrade_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LatestInfraVersion => {
                            if upgrade_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latestInfraVersion"));
                            }
                            upgrade_type__ = map_.next_value::<::std::option::Option<_>>()?.map(upgrade_node_group_request::UpgradeType::LatestInfraVersion)
;
                        }
                    }
                }
                Ok(UpgradeNodeGroupRequest {
                    id: id__.unwrap_or_default(),
                    upgrade_type: upgrade_type__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.mk8s.v1.UpgradeNodeGroupRequest", FIELDS, GeneratedVisitor)
    }
}
