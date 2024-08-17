pub mod proto {
    pub mod space_x {
        pub mod api {
            pub mod status {
                include!("space_x/space_x.api.status.rs");
            }

            pub mod device {
                include!("space_x/space_x.api.device.rs");
                pub mod services {
                    pub mod unlock {
                        include!("space_x/space_x.api.device.services.unlock.rs");
                    }
                }
            }

            pub mod satellites {
                pub mod network {
                    include!("space_x/space_x.api.satellites.network.rs");
                }
            }

            pub mod telemetron {
                pub mod public {
                    pub mod common {
                        include!("space_x/space_x.api.telemetron.public.common.rs");
                    }
                }
            }
        }
    }
}
