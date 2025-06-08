// @generated
impl serde::Serialize for PostgresqlConfig16 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.autovacuum_work_mem.is_some() {
            len += 1;
        }
        if self.statement_timeout.is_some() {
            len += 1;
        }
        if self.idle_in_transaction_session_timeout.is_some() {
            len += 1;
        }
        if self.autovacuum_vacuum_cost_delay.is_some() {
            len += 1;
        }
        if self.autovacuum_vacuum_cost_limit.is_some() {
            len += 1;
        }
        if self.autovacuum_naptime.is_some() {
            len += 1;
        }
        if self.autovacuum_vacuum_scale_factor.is_some() {
            len += 1;
        }
        if self.autovacuum_analyze_scale_factor.is_some() {
            len += 1;
        }
        if self.default_transaction_read_only.is_some() {
            len += 1;
        }
        if self.search_path.is_some() {
            len += 1;
        }
        if self.max_connections.is_some() {
            len += 1;
        }
        if self.shared_buffers.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.config.PostgresqlConfig16", len)?;
        if let Some(v) = self.autovacuum_work_mem.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("autovacuumWorkMem", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.statement_timeout.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("statementTimeout", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.idle_in_transaction_session_timeout.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("idleInTransactionSessionTimeout", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.autovacuum_vacuum_cost_delay.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("autovacuumVacuumCostDelay", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.autovacuum_vacuum_cost_limit.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("autovacuumVacuumCostLimit", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.autovacuum_naptime.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("autovacuumNaptime", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.autovacuum_vacuum_scale_factor.as_ref() {
            struct_ser.serialize_field("autovacuumVacuumScaleFactor", v)?;
        }
        if let Some(v) = self.autovacuum_analyze_scale_factor.as_ref() {
            struct_ser.serialize_field("autovacuumAnalyzeScaleFactor", v)?;
        }
        if let Some(v) = self.default_transaction_read_only.as_ref() {
            struct_ser.serialize_field("defaultTransactionReadOnly", v)?;
        }
        if let Some(v) = self.search_path.as_ref() {
            struct_ser.serialize_field("searchPath", v)?;
        }
        if let Some(v) = self.max_connections.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxConnections", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.shared_buffers.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sharedBuffers", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlConfig16 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "autovacuum_work_mem",
            "autovacuumWorkMem",
            "statement_timeout",
            "statementTimeout",
            "idle_in_transaction_session_timeout",
            "idleInTransactionSessionTimeout",
            "autovacuum_vacuum_cost_delay",
            "autovacuumVacuumCostDelay",
            "autovacuum_vacuum_cost_limit",
            "autovacuumVacuumCostLimit",
            "autovacuum_naptime",
            "autovacuumNaptime",
            "autovacuum_vacuum_scale_factor",
            "autovacuumVacuumScaleFactor",
            "autovacuum_analyze_scale_factor",
            "autovacuumAnalyzeScaleFactor",
            "default_transaction_read_only",
            "defaultTransactionReadOnly",
            "search_path",
            "searchPath",
            "max_connections",
            "maxConnections",
            "shared_buffers",
            "sharedBuffers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutovacuumWorkMem,
            StatementTimeout,
            IdleInTransactionSessionTimeout,
            AutovacuumVacuumCostDelay,
            AutovacuumVacuumCostLimit,
            AutovacuumNaptime,
            AutovacuumVacuumScaleFactor,
            AutovacuumAnalyzeScaleFactor,
            DefaultTransactionReadOnly,
            SearchPath,
            MaxConnections,
            SharedBuffers,
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
                            "autovacuumWorkMem" | "autovacuum_work_mem" => Ok(GeneratedField::AutovacuumWorkMem),
                            "statementTimeout" | "statement_timeout" => Ok(GeneratedField::StatementTimeout),
                            "idleInTransactionSessionTimeout" | "idle_in_transaction_session_timeout" => Ok(GeneratedField::IdleInTransactionSessionTimeout),
                            "autovacuumVacuumCostDelay" | "autovacuum_vacuum_cost_delay" => Ok(GeneratedField::AutovacuumVacuumCostDelay),
                            "autovacuumVacuumCostLimit" | "autovacuum_vacuum_cost_limit" => Ok(GeneratedField::AutovacuumVacuumCostLimit),
                            "autovacuumNaptime" | "autovacuum_naptime" => Ok(GeneratedField::AutovacuumNaptime),
                            "autovacuumVacuumScaleFactor" | "autovacuum_vacuum_scale_factor" => Ok(GeneratedField::AutovacuumVacuumScaleFactor),
                            "autovacuumAnalyzeScaleFactor" | "autovacuum_analyze_scale_factor" => Ok(GeneratedField::AutovacuumAnalyzeScaleFactor),
                            "defaultTransactionReadOnly" | "default_transaction_read_only" => Ok(GeneratedField::DefaultTransactionReadOnly),
                            "searchPath" | "search_path" => Ok(GeneratedField::SearchPath),
                            "maxConnections" | "max_connections" => Ok(GeneratedField::MaxConnections),
                            "sharedBuffers" | "shared_buffers" => Ok(GeneratedField::SharedBuffers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PostgresqlConfig16;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.config.PostgresqlConfig16")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PostgresqlConfig16, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut autovacuum_work_mem__ = None;
                let mut statement_timeout__ = None;
                let mut idle_in_transaction_session_timeout__ = None;
                let mut autovacuum_vacuum_cost_delay__ = None;
                let mut autovacuum_vacuum_cost_limit__ = None;
                let mut autovacuum_naptime__ = None;
                let mut autovacuum_vacuum_scale_factor__ = None;
                let mut autovacuum_analyze_scale_factor__ = None;
                let mut default_transaction_read_only__ = None;
                let mut search_path__ = None;
                let mut max_connections__ = None;
                let mut shared_buffers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutovacuumWorkMem => {
                            if autovacuum_work_mem__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autovacuumWorkMem"));
                            }
                            autovacuum_work_mem__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StatementTimeout => {
                            if statement_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statementTimeout"));
                            }
                            statement_timeout__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IdleInTransactionSessionTimeout => {
                            if idle_in_transaction_session_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleInTransactionSessionTimeout"));
                            }
                            idle_in_transaction_session_timeout__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutovacuumVacuumCostDelay => {
                            if autovacuum_vacuum_cost_delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autovacuumVacuumCostDelay"));
                            }
                            autovacuum_vacuum_cost_delay__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutovacuumVacuumCostLimit => {
                            if autovacuum_vacuum_cost_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autovacuumVacuumCostLimit"));
                            }
                            autovacuum_vacuum_cost_limit__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutovacuumNaptime => {
                            if autovacuum_naptime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autovacuumNaptime"));
                            }
                            autovacuum_naptime__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutovacuumVacuumScaleFactor => {
                            if autovacuum_vacuum_scale_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autovacuumVacuumScaleFactor"));
                            }
                            autovacuum_vacuum_scale_factor__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutovacuumAnalyzeScaleFactor => {
                            if autovacuum_analyze_scale_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autovacuumAnalyzeScaleFactor"));
                            }
                            autovacuum_analyze_scale_factor__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::DefaultTransactionReadOnly => {
                            if default_transaction_read_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultTransactionReadOnly"));
                            }
                            default_transaction_read_only__ = map_.next_value()?;
                        }
                        GeneratedField::SearchPath => {
                            if search_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchPath"));
                            }
                            search_path__ = map_.next_value()?;
                        }
                        GeneratedField::MaxConnections => {
                            if max_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConnections"));
                            }
                            max_connections__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SharedBuffers => {
                            if shared_buffers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sharedBuffers"));
                            }
                            shared_buffers__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(PostgresqlConfig16 {
                    autovacuum_work_mem: autovacuum_work_mem__,
                    statement_timeout: statement_timeout__,
                    idle_in_transaction_session_timeout: idle_in_transaction_session_timeout__,
                    autovacuum_vacuum_cost_delay: autovacuum_vacuum_cost_delay__,
                    autovacuum_vacuum_cost_limit: autovacuum_vacuum_cost_limit__,
                    autovacuum_naptime: autovacuum_naptime__,
                    autovacuum_vacuum_scale_factor: autovacuum_vacuum_scale_factor__,
                    autovacuum_analyze_scale_factor: autovacuum_analyze_scale_factor__,
                    default_transaction_read_only: default_transaction_read_only__,
                    search_path: search_path__,
                    max_connections: max_connections__,
                    shared_buffers: shared_buffers__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.config.PostgresqlConfig16", FIELDS, GeneratedVisitor)
    }
}
