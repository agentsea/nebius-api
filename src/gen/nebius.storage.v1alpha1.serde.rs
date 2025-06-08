// @generated
impl serde::Serialize for CreateTransferRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.CreateTransferRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTransferRequest {
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
            type Value = CreateTransferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.CreateTransferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTransferRequest, V::Error>
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
                Ok(CreateTransferRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.CreateTransferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTransferRequest {
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
        if self.resource_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.DeleteTransferRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.resource_version != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("resourceVersion", ToString::to_string(&self.resource_version).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTransferRequest {
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
            type Value = DeleteTransferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.DeleteTransferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTransferRequest, V::Error>
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
                            resource_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DeleteTransferRequest {
                    id: id__.unwrap_or_default(),
                    resource_version: resource_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.DeleteTransferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetIterationHistoryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transfer_id.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.GetIterationHistoryRequest", len)?;
        if !self.transfer_id.is_empty() {
            struct_ser.serialize_field("transferId", &self.transfer_id)?;
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
impl<'de> serde::Deserialize<'de> for GetIterationHistoryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transfer_id",
            "transferId",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransferId,
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
                            "transferId" | "transfer_id" => Ok(GeneratedField::TransferId),
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
            type Value = GetIterationHistoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.GetIterationHistoryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetIterationHistoryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transfer_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransferId => {
                            if transfer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transferId"));
                            }
                            transfer_id__ = Some(map_.next_value()?);
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
                Ok(GetIterationHistoryRequest {
                    transfer_id: transfer_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.GetIterationHistoryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetIterationHistoryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.iterations.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.GetIterationHistoryResponse", len)?;
        if !self.iterations.is_empty() {
            struct_ser.serialize_field("iterations", &self.iterations)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetIterationHistoryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "iterations",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Iterations,
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
                            "iterations" => Ok(GeneratedField::Iterations),
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
            type Value = GetIterationHistoryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.GetIterationHistoryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetIterationHistoryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut iterations__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Iterations => {
                            if iterations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iterations"));
                            }
                            iterations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetIterationHistoryResponse {
                    iterations: iterations__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.GetIterationHistoryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTransferRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.GetTransferRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTransferRequest {
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
            type Value = GetTransferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.GetTransferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTransferRequest, V::Error>
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
                Ok(GetTransferRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.GetTransferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTransfersRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.ListTransfersRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListTransfersRequest {
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
            type Value = ListTransfersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.ListTransfersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTransfersRequest, V::Error>
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
                Ok(ListTransfersRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.ListTransfersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTransfersResponse {
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
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.ListTransfersResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTransfersResponse {
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
            type Value = ListTransfersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.ListTransfersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTransfersResponse, V::Error>
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
                Ok(ListTransfersResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.ListTransfersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResumeTransferRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.ResumeTransferRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResumeTransferRequest {
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
            type Value = ResumeTransferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.ResumeTransferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResumeTransferRequest, V::Error>
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
                Ok(ResumeTransferRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.ResumeTransferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StopTransferRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.StopTransferRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StopTransferRequest {
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
            type Value = StopTransferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.StopTransferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StopTransferRequest, V::Error>
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
                Ok(StopTransferRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.StopTransferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Transfer {
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
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.Transfer", len)?;
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
impl<'de> serde::Deserialize<'de> for Transfer {
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
            type Value = Transfer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.Transfer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Transfer, V::Error>
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
                Ok(Transfer {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.Transfer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransferIteration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sequence_number != 0 {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if !self.error.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if self.objects_discovered_count != 0 {
            len += 1;
        }
        if self.objects_migrated_count != 0 {
            len += 1;
        }
        if self.objects_skipped_count != 0 {
            len += 1;
        }
        if self.objects_migrated_size != 0 {
            len += 1;
        }
        if self.average_throughput_bytes != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferIteration", len)?;
        if self.sequence_number != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sequenceNumber", ToString::to_string(&self.sequence_number).as_str())?;
        }
        if self.state != 0 {
            let v = transfer_iteration::State::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.error.is_empty() {
            struct_ser.serialize_field("error", &self.error)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if self.objects_discovered_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("objectsDiscoveredCount", ToString::to_string(&self.objects_discovered_count).as_str())?;
        }
        if self.objects_migrated_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("objectsMigratedCount", ToString::to_string(&self.objects_migrated_count).as_str())?;
        }
        if self.objects_skipped_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("objectsSkippedCount", ToString::to_string(&self.objects_skipped_count).as_str())?;
        }
        if self.objects_migrated_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("objectsMigratedSize", ToString::to_string(&self.objects_migrated_size).as_str())?;
        }
        if self.average_throughput_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("averageThroughputBytes", ToString::to_string(&self.average_throughput_bytes).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransferIteration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sequence_number",
            "sequenceNumber",
            "state",
            "error",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "objects_discovered_count",
            "objectsDiscoveredCount",
            "objects_migrated_count",
            "objectsMigratedCount",
            "objects_skipped_count",
            "objectsSkippedCount",
            "objects_migrated_size",
            "objectsMigratedSize",
            "average_throughput_bytes",
            "averageThroughputBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SequenceNumber,
            State,
            Error,
            StartTime,
            EndTime,
            ObjectsDiscoveredCount,
            ObjectsMigratedCount,
            ObjectsSkippedCount,
            ObjectsMigratedSize,
            AverageThroughputBytes,
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
                            "sequenceNumber" | "sequence_number" => Ok(GeneratedField::SequenceNumber),
                            "state" => Ok(GeneratedField::State),
                            "error" => Ok(GeneratedField::Error),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "objectsDiscoveredCount" | "objects_discovered_count" => Ok(GeneratedField::ObjectsDiscoveredCount),
                            "objectsMigratedCount" | "objects_migrated_count" => Ok(GeneratedField::ObjectsMigratedCount),
                            "objectsSkippedCount" | "objects_skipped_count" => Ok(GeneratedField::ObjectsSkippedCount),
                            "objectsMigratedSize" | "objects_migrated_size" => Ok(GeneratedField::ObjectsMigratedSize),
                            "averageThroughputBytes" | "average_throughput_bytes" => Ok(GeneratedField::AverageThroughputBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransferIteration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferIteration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransferIteration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sequence_number__ = None;
                let mut state__ = None;
                let mut error__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut objects_discovered_count__ = None;
                let mut objects_migrated_count__ = None;
                let mut objects_skipped_count__ = None;
                let mut objects_migrated_size__ = None;
                let mut average_throughput_bytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SequenceNumber => {
                            if sequence_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceNumber"));
                            }
                            sequence_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<transfer_iteration::State>()? as i32);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::ObjectsDiscoveredCount => {
                            if objects_discovered_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectsDiscoveredCount"));
                            }
                            objects_discovered_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ObjectsMigratedCount => {
                            if objects_migrated_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectsMigratedCount"));
                            }
                            objects_migrated_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ObjectsSkippedCount => {
                            if objects_skipped_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectsSkippedCount"));
                            }
                            objects_skipped_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ObjectsMigratedSize => {
                            if objects_migrated_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectsMigratedSize"));
                            }
                            objects_migrated_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AverageThroughputBytes => {
                            if average_throughput_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("averageThroughputBytes"));
                            }
                            average_throughput_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TransferIteration {
                    sequence_number: sequence_number__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    error: error__.unwrap_or_default(),
                    start_time: start_time__,
                    end_time: end_time__,
                    objects_discovered_count: objects_discovered_count__.unwrap_or_default(),
                    objects_migrated_count: objects_migrated_count__.unwrap_or_default(),
                    objects_skipped_count: objects_skipped_count__.unwrap_or_default(),
                    objects_migrated_size: objects_migrated_size__.unwrap_or_default(),
                    average_throughput_bytes: average_throughput_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferIteration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_iteration::State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATE_UNSPECIFIED",
            Self::InProgress => "IN_PROGRESS",
            Self::Completed => "COMPLETED",
            Self::Stopped => "STOPPED",
            Self::Failed => "FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for transfer_iteration::State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_UNSPECIFIED",
            "IN_PROGRESS",
            "COMPLETED",
            "STOPPED",
            "FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transfer_iteration::State;

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
                    "STATE_UNSPECIFIED" => Ok(transfer_iteration::State::Unspecified),
                    "IN_PROGRESS" => Ok(transfer_iteration::State::InProgress),
                    "COMPLETED" => Ok(transfer_iteration::State::Completed),
                    "STOPPED" => Ok(transfer_iteration::State::Stopped),
                    "FAILED" => Ok(transfer_iteration::State::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TransferSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if self.destination.is_some() {
            len += 1;
        }
        if self.inter_iteration_interval.is_some() {
            len += 1;
        }
        if self.overwrite_strategy != 0 {
            len += 1;
        }
        if self.stop_condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferSpec", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if let Some(v) = self.destination.as_ref() {
            struct_ser.serialize_field("destination", v)?;
        }
        if let Some(v) = self.inter_iteration_interval.as_ref() {
            struct_ser.serialize_field("interIterationInterval", v)?;
        }
        if self.overwrite_strategy != 0 {
            let v = transfer_spec::OverwriteStrategy::try_from(self.overwrite_strategy)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.overwrite_strategy)))?;
            struct_ser.serialize_field("overwriteStrategy", &v)?;
        }
        if let Some(v) = self.stop_condition.as_ref() {
            match v {
                transfer_spec::StopCondition::AfterOneIteration(v) => {
                    struct_ser.serialize_field("afterOneIteration", v)?;
                }
                transfer_spec::StopCondition::AfterNEmptyIterations(v) => {
                    struct_ser.serialize_field("afterNEmptyIterations", v)?;
                }
                transfer_spec::StopCondition::Infinite(v) => {
                    struct_ser.serialize_field("infinite", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransferSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "destination",
            "inter_iteration_interval",
            "interIterationInterval",
            "overwrite_strategy",
            "overwriteStrategy",
            "after_one_iteration",
            "afterOneIteration",
            "after_n_empty_iterations",
            "afterNEmptyIterations",
            "infinite",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Destination,
            InterIterationInterval,
            OverwriteStrategy,
            AfterOneIteration,
            AfterNEmptyIterations,
            Infinite,
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
                            "source" => Ok(GeneratedField::Source),
                            "destination" => Ok(GeneratedField::Destination),
                            "interIterationInterval" | "inter_iteration_interval" => Ok(GeneratedField::InterIterationInterval),
                            "overwriteStrategy" | "overwrite_strategy" => Ok(GeneratedField::OverwriteStrategy),
                            "afterOneIteration" | "after_one_iteration" => Ok(GeneratedField::AfterOneIteration),
                            "afterNEmptyIterations" | "after_n_empty_iterations" => Ok(GeneratedField::AfterNEmptyIterations),
                            "infinite" => Ok(GeneratedField::Infinite),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransferSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransferSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                let mut destination__ = None;
                let mut inter_iteration_interval__ = None;
                let mut overwrite_strategy__ = None;
                let mut stop_condition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = map_.next_value()?;
                        }
                        GeneratedField::Destination => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            destination__ = map_.next_value()?;
                        }
                        GeneratedField::InterIterationInterval => {
                            if inter_iteration_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interIterationInterval"));
                            }
                            inter_iteration_interval__ = map_.next_value()?;
                        }
                        GeneratedField::OverwriteStrategy => {
                            if overwrite_strategy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overwriteStrategy"));
                            }
                            overwrite_strategy__ = Some(map_.next_value::<transfer_spec::OverwriteStrategy>()? as i32);
                        }
                        GeneratedField::AfterOneIteration => {
                            if stop_condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("afterOneIteration"));
                            }
                            stop_condition__ = map_.next_value::<::std::option::Option<_>>()?.map(transfer_spec::StopCondition::AfterOneIteration)
;
                        }
                        GeneratedField::AfterNEmptyIterations => {
                            if stop_condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("afterNEmptyIterations"));
                            }
                            stop_condition__ = map_.next_value::<::std::option::Option<_>>()?.map(transfer_spec::StopCondition::AfterNEmptyIterations)
;
                        }
                        GeneratedField::Infinite => {
                            if stop_condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("infinite"));
                            }
                            stop_condition__ = map_.next_value::<::std::option::Option<_>>()?.map(transfer_spec::StopCondition::Infinite)
;
                        }
                    }
                }
                Ok(TransferSpec {
                    source: source__,
                    destination: destination__,
                    inter_iteration_interval: inter_iteration_interval__,
                    overwrite_strategy: overwrite_strategy__.unwrap_or_default(),
                    stop_condition: stop_condition__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_spec::BucketCredentials {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.credentials.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferSpec.BucketCredentials", len)?;
        if let Some(v) = self.credentials.as_ref() {
            match v {
                transfer_spec::bucket_credentials::Credentials::Anonymous(v) => {
                    struct_ser.serialize_field("anonymous", v)?;
                }
                transfer_spec::bucket_credentials::Credentials::AccessKey(v) => {
                    struct_ser.serialize_field("accessKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transfer_spec::BucketCredentials {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "anonymous",
            "access_key",
            "accessKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Anonymous,
            AccessKey,
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
                            "anonymous" => Ok(GeneratedField::Anonymous),
                            "accessKey" | "access_key" => Ok(GeneratedField::AccessKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transfer_spec::BucketCredentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferSpec.BucketCredentials")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<transfer_spec::BucketCredentials, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credentials__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Anonymous => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anonymous"));
                            }
                            credentials__ = map_.next_value::<::std::option::Option<_>>()?.map(transfer_spec::bucket_credentials::Credentials::Anonymous)
;
                        }
                        GeneratedField::AccessKey => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessKey"));
                            }
                            credentials__ = map_.next_value::<::std::option::Option<_>>()?.map(transfer_spec::bucket_credentials::Credentials::AccessKey)
;
                        }
                    }
                }
                Ok(transfer_spec::BucketCredentials {
                    credentials: credentials__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferSpec.BucketCredentials", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_spec::bucket_credentials::CredentialsAccessKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_key_id.is_empty() {
            len += 1;
        }
        if !self.secret_access_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferSpec.BucketCredentials.CredentialsAccessKey", len)?;
        if !self.access_key_id.is_empty() {
            struct_ser.serialize_field("accessKeyId", &self.access_key_id)?;
        }
        if !self.secret_access_key.is_empty() {
            struct_ser.serialize_field("secretAccessKey", &self.secret_access_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transfer_spec::bucket_credentials::CredentialsAccessKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_key_id",
            "accessKeyId",
            "secret_access_key",
            "secretAccessKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessKeyId,
            SecretAccessKey,
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
                            "accessKeyId" | "access_key_id" => Ok(GeneratedField::AccessKeyId),
                            "secretAccessKey" | "secret_access_key" => Ok(GeneratedField::SecretAccessKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transfer_spec::bucket_credentials::CredentialsAccessKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferSpec.BucketCredentials.CredentialsAccessKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<transfer_spec::bucket_credentials::CredentialsAccessKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_key_id__ = None;
                let mut secret_access_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessKeyId => {
                            if access_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessKeyId"));
                            }
                            access_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SecretAccessKey => {
                            if secret_access_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secretAccessKey"));
                            }
                            secret_access_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(transfer_spec::bucket_credentials::CredentialsAccessKey {
                    access_key_id: access_key_id__.unwrap_or_default(),
                    secret_access_key: secret_access_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferSpec.BucketCredentials.CredentialsAccessKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_spec::bucket_credentials::CredentialsAnonymous {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferSpec.BucketCredentials.CredentialsAnonymous", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transfer_spec::bucket_credentials::CredentialsAnonymous {
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
            type Value = transfer_spec::bucket_credentials::CredentialsAnonymous;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferSpec.BucketCredentials.CredentialsAnonymous")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<transfer_spec::bucket_credentials::CredentialsAnonymous, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(transfer_spec::bucket_credentials::CredentialsAnonymous {
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferSpec.BucketCredentials.CredentialsAnonymous", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_spec::DestinationBucket {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bucket_name.is_empty() {
            len += 1;
        }
        if self.credentials.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferSpec.DestinationBucket", len)?;
        if !self.bucket_name.is_empty() {
            struct_ser.serialize_field("bucketName", &self.bucket_name)?;
        }
        if let Some(v) = self.credentials.as_ref() {
            struct_ser.serialize_field("credentials", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transfer_spec::DestinationBucket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bucket_name",
            "bucketName",
            "credentials",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BucketName,
            Credentials,
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
                            "bucketName" | "bucket_name" => Ok(GeneratedField::BucketName),
                            "credentials" => Ok(GeneratedField::Credentials),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transfer_spec::DestinationBucket;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferSpec.DestinationBucket")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<transfer_spec::DestinationBucket, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bucket_name__ = None;
                let mut credentials__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BucketName => {
                            if bucket_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketName"));
                            }
                            bucket_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Credentials => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            credentials__ = map_.next_value()?;
                        }
                    }
                }
                Ok(transfer_spec::DestinationBucket {
                    bucket_name: bucket_name__.unwrap_or_default(),
                    credentials: credentials__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferSpec.DestinationBucket", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_spec::OverwriteStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OVERWRITE_STRATEGY_UNSPECIFIED",
            Self::Never => "NEVER",
            Self::IfNewer => "IF_NEWER",
            Self::Always => "ALWAYS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for transfer_spec::OverwriteStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OVERWRITE_STRATEGY_UNSPECIFIED",
            "NEVER",
            "IF_NEWER",
            "ALWAYS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transfer_spec::OverwriteStrategy;

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
                    "OVERWRITE_STRATEGY_UNSPECIFIED" => Ok(transfer_spec::OverwriteStrategy::Unspecified),
                    "NEVER" => Ok(transfer_spec::OverwriteStrategy::Never),
                    "IF_NEWER" => Ok(transfer_spec::OverwriteStrategy::IfNewer),
                    "ALWAYS" => Ok(transfer_spec::OverwriteStrategy::Always),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_spec::SourceBucket {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.endpoint.is_empty() {
            len += 1;
        }
        if !self.bucket_name.is_empty() {
            len += 1;
        }
        if !self.region.is_empty() {
            len += 1;
        }
        if self.credentials.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferSpec.SourceBucket", len)?;
        if !self.endpoint.is_empty() {
            struct_ser.serialize_field("endpoint", &self.endpoint)?;
        }
        if !self.bucket_name.is_empty() {
            struct_ser.serialize_field("bucketName", &self.bucket_name)?;
        }
        if !self.region.is_empty() {
            struct_ser.serialize_field("region", &self.region)?;
        }
        if let Some(v) = self.credentials.as_ref() {
            struct_ser.serialize_field("credentials", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transfer_spec::SourceBucket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint",
            "bucket_name",
            "bucketName",
            "region",
            "credentials",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Endpoint,
            BucketName,
            Region,
            Credentials,
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
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "bucketName" | "bucket_name" => Ok(GeneratedField::BucketName),
                            "region" => Ok(GeneratedField::Region),
                            "credentials" => Ok(GeneratedField::Credentials),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transfer_spec::SourceBucket;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferSpec.SourceBucket")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<transfer_spec::SourceBucket, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint__ = None;
                let mut bucket_name__ = None;
                let mut region__ = None;
                let mut credentials__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BucketName => {
                            if bucket_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketName"));
                            }
                            bucket_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Region => {
                            if region__.is_some() {
                                return Err(serde::de::Error::duplicate_field("region"));
                            }
                            region__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Credentials => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            credentials__ = map_.next_value()?;
                        }
                    }
                }
                Ok(transfer_spec::SourceBucket {
                    endpoint: endpoint__.unwrap_or_default(),
                    bucket_name: bucket_name__.unwrap_or_default(),
                    region: region__.unwrap_or_default(),
                    credentials: credentials__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferSpec.SourceBucket", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_spec::StopConditionAfterNEmptyIterations {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.empty_iterations_threshold != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferSpec.StopConditionAfterNEmptyIterations", len)?;
        if self.empty_iterations_threshold != 0 {
            struct_ser.serialize_field("emptyIterationsThreshold", &self.empty_iterations_threshold)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transfer_spec::StopConditionAfterNEmptyIterations {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "empty_iterations_threshold",
            "emptyIterationsThreshold",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EmptyIterationsThreshold,
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
                            "emptyIterationsThreshold" | "empty_iterations_threshold" => Ok(GeneratedField::EmptyIterationsThreshold),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transfer_spec::StopConditionAfterNEmptyIterations;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferSpec.StopConditionAfterNEmptyIterations")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<transfer_spec::StopConditionAfterNEmptyIterations, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut empty_iterations_threshold__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EmptyIterationsThreshold => {
                            if empty_iterations_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emptyIterationsThreshold"));
                            }
                            empty_iterations_threshold__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(transfer_spec::StopConditionAfterNEmptyIterations {
                    empty_iterations_threshold: empty_iterations_threshold__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferSpec.StopConditionAfterNEmptyIterations", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_spec::StopConditionAfterOneIteration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferSpec.StopConditionAfterOneIteration", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transfer_spec::StopConditionAfterOneIteration {
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
            type Value = transfer_spec::StopConditionAfterOneIteration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferSpec.StopConditionAfterOneIteration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<transfer_spec::StopConditionAfterOneIteration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(transfer_spec::StopConditionAfterOneIteration {
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferSpec.StopConditionAfterOneIteration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_spec::StopConditionInfinite {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferSpec.StopConditionInfinite", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transfer_spec::StopConditionInfinite {
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
            type Value = transfer_spec::StopConditionInfinite;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferSpec.StopConditionInfinite")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<transfer_spec::StopConditionInfinite, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(transfer_spec::StopConditionInfinite {
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferSpec.StopConditionInfinite", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransferStatus {
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
        if !self.error.is_empty() {
            len += 1;
        }
        if self.suspension_state != 0 {
            len += 1;
        }
        if self.last_iteration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.TransferStatus", len)?;
        if self.state != 0 {
            let v = transfer_status::State::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.error.is_empty() {
            struct_ser.serialize_field("error", &self.error)?;
        }
        if self.suspension_state != 0 {
            let v = transfer_status::SuspensionState::try_from(self.suspension_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.suspension_state)))?;
            struct_ser.serialize_field("suspensionState", &v)?;
        }
        if let Some(v) = self.last_iteration.as_ref() {
            struct_ser.serialize_field("lastIteration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransferStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "error",
            "suspension_state",
            "suspensionState",
            "last_iteration",
            "lastIteration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            Error,
            SuspensionState,
            LastIteration,
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
                            "error" => Ok(GeneratedField::Error),
                            "suspensionState" | "suspension_state" => Ok(GeneratedField::SuspensionState),
                            "lastIteration" | "last_iteration" => Ok(GeneratedField::LastIteration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransferStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.TransferStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransferStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut error__ = None;
                let mut suspension_state__ = None;
                let mut last_iteration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<transfer_status::State>()? as i32);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SuspensionState => {
                            if suspension_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suspensionState"));
                            }
                            suspension_state__ = Some(map_.next_value::<transfer_status::SuspensionState>()? as i32);
                        }
                        GeneratedField::LastIteration => {
                            if last_iteration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastIteration"));
                            }
                            last_iteration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TransferStatus {
                    state: state__.unwrap_or_default(),
                    error: error__.unwrap_or_default(),
                    suspension_state: suspension_state__.unwrap_or_default(),
                    last_iteration: last_iteration__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.TransferStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_status::State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATE_UNSPECIFIED",
            Self::Active => "ACTIVE",
            Self::Stopping => "STOPPING",
            Self::Stopped => "STOPPED",
            Self::Failing => "FAILING",
            Self::Failed => "FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for transfer_status::State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_UNSPECIFIED",
            "ACTIVE",
            "STOPPING",
            "STOPPED",
            "FAILING",
            "FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transfer_status::State;

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
                    "STATE_UNSPECIFIED" => Ok(transfer_status::State::Unspecified),
                    "ACTIVE" => Ok(transfer_status::State::Active),
                    "STOPPING" => Ok(transfer_status::State::Stopping),
                    "STOPPED" => Ok(transfer_status::State::Stopped),
                    "FAILING" => Ok(transfer_status::State::Failing),
                    "FAILED" => Ok(transfer_status::State::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for transfer_status::SuspensionState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SUSPENSION_STATE_UNSPECIFIED",
            Self::NotSuspended => "NOT_SUSPENDED",
            Self::Suspended => "SUSPENDED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for transfer_status::SuspensionState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SUSPENSION_STATE_UNSPECIFIED",
            "NOT_SUSPENDED",
            "SUSPENDED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transfer_status::SuspensionState;

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
                    "SUSPENSION_STATE_UNSPECIFIED" => Ok(transfer_status::SuspensionState::Unspecified),
                    "NOT_SUSPENDED" => Ok(transfer_status::SuspensionState::NotSuspended),
                    "SUSPENDED" => Ok(transfer_status::SuspensionState::Suspended),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTransferRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1alpha1.UpdateTransferRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTransferRequest {
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
            type Value = UpdateTransferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1alpha1.UpdateTransferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTransferRequest, V::Error>
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
                Ok(UpdateTransferRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1alpha1.UpdateTransferRequest", FIELDS, GeneratedVisitor)
    }
}
