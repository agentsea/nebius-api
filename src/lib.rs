// src/lib.rs
#![allow(clippy::all)] // silence warnings from generated code

pub mod nebius {
    pub mod common {
        pub mod v1 {
            include!("gen/nebius.common.v1.rs");
            include!("gen/nebius.common.v1.serde.rs");
            include!("gen/nebius.common.v1.tonic.rs");
        }
        pub mod v1alpha1 {
            include!("gen/nebius.common.v1alpha1.rs");
            include!("gen/nebius.common.v1alpha1.serde.rs");
            include!("gen/nebius.common.v1alpha1.tonic.rs");
        }
        pub mod error {
            pub mod v1alpha1 {
                include!("gen/nebius.common.error.v1alpha1.rs");
                include!("gen/nebius.common.error.v1alpha1.serde.rs");
            }
        }
    }

    pub mod storage {
        pub mod v1 {
            include!("gen/nebius.storage.v1.rs");
            include!("gen/nebius.storage.v1.serde.rs");
            include!("gen/nebius.storage.v1.tonic.rs");
        }
        pub mod v1alpha1 {
            include!("gen/nebius.storage.v1alpha1.rs");
            include!("gen/nebius.storage.v1alpha1.serde.rs");
            include!("gen/nebius.storage.v1alpha1.tonic.rs");
        }
    }

    pub mod vpc {
        pub mod v1 {
            include!("gen/nebius.vpc.v1.rs");
            include!("gen/nebius.vpc.v1.serde.rs");
            include!("gen/nebius.vpc.v1.tonic.rs");
        }
        pub mod v1alpha1 {
            include!("gen/nebius.vpc.v1alpha1.rs");
            include!("gen/nebius.vpc.v1alpha1.serde.rs");
            include!("gen/nebius.vpc.v1alpha1.tonic.rs");
        }
    }

    // …repeat for iam, msp, compute, etc.  Just follow the same pattern …

    // Root-level definitions such as RegionRouting, DeprecationDetails, etc.
    include!("gen/nebius.rs");

    // ----- additional generated packages -----

    pub mod registry {
        pub mod v1 {
            include!("gen/nebius.registry.v1.rs");
            include!("gen/nebius.registry.v1.serde.rs");
            include!("gen/nebius.registry.v1.tonic.rs");
        }
    }

    pub mod quotas {
        pub mod v1 {
            include!("gen/nebius.quotas.v1.rs");
            include!("gen/nebius.quotas.v1.serde.rs");
            include!("gen/nebius.quotas.v1.tonic.rs");
        }
    }

    pub mod iam {
        pub mod v1 {
            include!("gen/nebius.iam.v1.rs");
            include!("gen/nebius.iam.v1.serde.rs");
            include!("gen/nebius.iam.v1.tonic.rs");
        }
        pub mod v2 {
            include!("gen/nebius.iam.v2.rs");
            include!("gen/nebius.iam.v2.serde.rs");
            include!("gen/nebius.iam.v2.tonic.rs");
        }
    }

    pub mod mk8s {
        pub mod v1 {
            include!("gen/nebius.mk8s.v1.rs");
            include!("gen/nebius.mk8s.v1.serde.rs");
            include!("gen/nebius.mk8s.v1.tonic.rs");
        }
        pub mod v1alpha1 {
            include!("gen/nebius.mk8s.v1alpha1.rs");
            include!("gen/nebius.mk8s.v1alpha1.serde.rs");
            include!("gen/nebius.mk8s.v1alpha1.tonic.rs");
        }
    }

    pub mod compute {
        pub mod v1 {
            include!("gen/nebius.compute.v1.rs");
            include!("gen/nebius.compute.v1.serde.rs");
            include!("gen/nebius.compute.v1.tonic.rs");
        }
        pub mod v1alpha1 {
            include!("gen/nebius.compute.v1alpha1.rs");
            include!("gen/nebius.compute.v1alpha1.serde.rs");
            include!("gen/nebius.compute.v1alpha1.tonic.rs");
        }
    }

    pub mod logging {
        pub mod agentmanager {
            pub mod v1 {
                include!("gen/nebius.logging.agentmanager.v1.rs");
                include!("gen/nebius.logging.agentmanager.v1.serde.rs");
                include!("gen/nebius.logging.agentmanager.v1.tonic.rs");
            }
        }
    }

    pub mod applications {
        pub mod v1alpha1 {
            include!("gen/nebius.applications.v1alpha1.rs");
            include!("gen/nebius.applications.v1alpha1.serde.rs");
            include!("gen/nebius.applications.v1alpha1.tonic.rs");
        }
    }

    pub mod audit {
        pub mod v2 {
            include!("gen/nebius.audit.v2.rs");
            include!("gen/nebius.audit.v2.serde.rs");
            include!("gen/nebius.audit.v2.tonic.rs");
        }
    }

    pub mod msp {
        pub mod v1alpha1 {
            include!("gen/nebius.msp.v1alpha1.rs");
            include!("gen/nebius.msp.v1alpha1.serde.rs");
            // NOTE: there is no generated tonic file for plain v1alpha1

            pub mod resource {
                include!("gen/nebius.msp.v1alpha1.resource.rs");
                include!("gen/nebius.msp.v1alpha1.resource.serde.rs");
                include!("gen/nebius.msp.v1alpha1.resource.tonic.rs");
            }
        }

        pub mod spark {
            pub mod v1alpha1 {
                include!("gen/nebius.msp.spark.v1alpha1.rs");
                include!("gen/nebius.msp.spark.v1alpha1.serde.rs");
                include!("gen/nebius.msp.spark.v1alpha1.tonic.rs");
            }
        }

        pub mod postgresql {
            pub mod v1alpha1 {
                include!("gen/nebius.msp.postgresql.v1alpha1.rs");
                include!("gen/nebius.msp.postgresql.v1alpha1.serde.rs");
                include!("gen/nebius.msp.postgresql.v1alpha1.tonic.rs");

                pub mod config {
                    include!("gen/nebius.msp.postgresql.v1alpha1.config.rs");
                    include!("gen/nebius.msp.postgresql.v1alpha1.config.serde.rs");
                }
            }
        }

        pub mod serverless {
            pub mod v1alpha1 {
                include!("gen/nebius.msp.serverless.v1alpha1.rs");
                include!("gen/nebius.msp.serverless.v1alpha1.serde.rs");
                include!("gen/nebius.msp.serverless.v1alpha1.tonic.rs");
            }
        }

        pub mod mlflow {
            pub mod v1alpha1 {
                include!("gen/nebius.msp.mlflow.v1alpha1.rs");
                include!("gen/nebius.msp.mlflow.v1alpha1.serde.rs");
                include!("gen/nebius.msp.mlflow.v1alpha1.tonic.rs");
            }
        }
    }
}
