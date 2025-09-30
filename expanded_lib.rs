#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod components {
    pub mod bot {
        use super::prelude::*;
        use kenzu::Builder;
        use nekoprint::NekoPrint;
        use serde::{Deserialize, Serialize};
        #[transporter(async fn procedure(){println!("{message}");})]
        pub struct NekoBot {
            #[set(value = WebDriver::new())]
            pub web_driver: WebDriver,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NekoBot {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "NekoBot",
                    "web_driver",
                    &&self.web_driver,
                )
            }
        }
        impl NekoBot {
            pub fn new() -> Self {
                Self {
                    web_driver: WebDriver::new().into(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    web_driver: instance.web_driver,
                }
            }
            pub fn web_driver<New: Into<WebDriver> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.web_driver = new.into();
                self
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for NekoBot {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "NekoBot",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "web_driver",
                        &self.web_driver,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for NekoBot {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private228::Ok(__Field::__field0),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "web_driver" => _serde::__private228::Ok(__Field::__field0),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"web_driver" => _serde::__private228::Ok(__Field::__field0),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private228::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private228::PhantomData<NekoBot>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = NekoBot;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct NekoBot",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                WebDriver,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct NekoBot with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(NekoBot { web_driver: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private228::Option<WebDriver> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "web_driver",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<WebDriver>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("web_driver")?
                                }
                            };
                            _serde::__private228::Ok(NekoBot { web_driver: __field0 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["web_driver"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "NekoBot",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<NekoBot>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Neko_Print_Printer_NekoBot_Web_driver {
            pub target: WebDriver,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_NekoBot_Web_driver {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_NekoBot_Web_driver",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_NekoBot_Web_driver {
            #[inline]
            fn default() -> Neko_Print_Printer_NekoBot_Web_driver {
                Neko_Print_Printer_NekoBot_Web_driver {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_NekoBot_Web_driver {
            pub fn new() -> Self {
                Self {
                    target: <WebDriver as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<WebDriver> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_NekoBot_Web_driver {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                "NekoBot",
                                "web_driver",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                "NekoBot",
                                "web_driver",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                "NekoBot",
                                "web_driver",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                "NekoBot",
                                "web_driver",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                "NekoBot",
                                "web_driver",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                "NekoBot",
                                "web_driver",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                "NekoBot",
                                "web_driver",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl NekoBot {
            pub fn print_web_driver(&self) -> Neko_Print_Printer_NekoBot_Web_driver {
                Neko_Print_Printer_NekoBot_Web_driver::new()
                    .target(self.web_driver.clone())
            }
        }
        pub struct Neko_Print_Printer_NekoBot {
            pub target: NekoBot,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_NekoBot {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_NekoBot",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_NekoBot {
            #[inline]
            fn default() -> Neko_Print_Printer_NekoBot {
                Neko_Print_Printer_NekoBot {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_NekoBot {
            pub fn new() -> Self {
                Self {
                    target: <NekoBot as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<NekoBot> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_NekoBot {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/bot.rs",
                                6u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
        }
        impl NekoBot {
            pub fn print(&self) -> Neko_Print_Printer_NekoBot {
                Neko_Print_Printer_NekoBot::new().target(self.clone())
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for NekoBot {
            #[inline]
            fn clone(&self) -> NekoBot {
                NekoBot {
                    web_driver: ::core::clone::Clone::clone(&self.web_driver),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for NekoBot {
            #[inline]
            fn default() -> NekoBot {
                NekoBot {
                    web_driver: ::core::default::Default::default(),
                }
            }
        }
        impl NekoBot {
            pub fn firefox() -> Self {
                Self::new().web_driver(WebDriver::new().browser(Browser::Firefox))
            }
            pub fn chrome() -> Self {
                Self::new()
            }
        }
    }
    pub mod prelude {
        pub use super::bot::*;
        pub use super::web_driver::*;
    }
    pub mod web_driver {
        use kenzu::Builder;
        use nekoprint::NekoPrint;
        use reqwest::{Client, Error};
        use serde::{Deserialize, Serialize};
        use serde_json::{Value, json};
        #[transporter(async fn procedure(){println!("{message}");})]
        pub struct Args {
            pub binary: String,
            pub headless: bool,
            pub sandbox: bool,
            pub disable_gpu: bool,
            pub user_data_dir: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Args {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Args",
                    "binary",
                    &self.binary,
                    "headless",
                    &self.headless,
                    "sandbox",
                    &self.sandbox,
                    "disable_gpu",
                    &self.disable_gpu,
                    "user_data_dir",
                    &&self.user_data_dir,
                )
            }
        }
        impl Args {
            pub fn new() -> Self {
                Self {
                    binary: <String as Default>::default(),
                    headless: <bool as Default>::default(),
                    sandbox: <bool as Default>::default(),
                    disable_gpu: <bool as Default>::default(),
                    user_data_dir: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    binary: instance.binary,
                    headless: instance.headless,
                    sandbox: instance.sandbox,
                    disable_gpu: instance.disable_gpu,
                    user_data_dir: instance.user_data_dir,
                }
            }
            pub fn binary<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.binary = new.into();
                self
            }
            pub fn headless<New: Into<bool> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.headless = new.into();
                self
            }
            pub fn sandbox<New: Into<bool> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.sandbox = new.into();
                self
            }
            pub fn disable_gpu<New: Into<bool> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.disable_gpu = new.into();
                self
            }
            pub fn user_data_dir<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.user_data_dir = new.into();
                self
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Args {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Args",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "binary",
                        &self.binary,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "headless",
                        &self.headless,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "sandbox",
                        &self.sandbox,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "disable_gpu",
                        &self.disable_gpu,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "user_data_dir",
                        &self.user_data_dir,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Args {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private228::Ok(__Field::__field0),
                                1u64 => _serde::__private228::Ok(__Field::__field1),
                                2u64 => _serde::__private228::Ok(__Field::__field2),
                                3u64 => _serde::__private228::Ok(__Field::__field3),
                                4u64 => _serde::__private228::Ok(__Field::__field4),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "binary" => _serde::__private228::Ok(__Field::__field0),
                                "headless" => _serde::__private228::Ok(__Field::__field1),
                                "sandbox" => _serde::__private228::Ok(__Field::__field2),
                                "disable_gpu" => _serde::__private228::Ok(__Field::__field3),
                                "user_data_dir" => {
                                    _serde::__private228::Ok(__Field::__field4)
                                }
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"binary" => _serde::__private228::Ok(__Field::__field0),
                                b"headless" => _serde::__private228::Ok(__Field::__field1),
                                b"sandbox" => _serde::__private228::Ok(__Field::__field2),
                                b"disable_gpu" => {
                                    _serde::__private228::Ok(__Field::__field3)
                                }
                                b"user_data_dir" => {
                                    _serde::__private228::Ok(__Field::__field4)
                                }
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private228::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private228::PhantomData<Args>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Args;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct Args",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Args with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Args with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Args with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Args with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Args with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(Args {
                                binary: __field0,
                                headless: __field1,
                                sandbox: __field2,
                                disable_gpu: __field3,
                                user_data_dir: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                            let mut __field1: _serde::__private228::Option<bool> = _serde::__private228::None;
                            let mut __field2: _serde::__private228::Option<bool> = _serde::__private228::None;
                            let mut __field3: _serde::__private228::Option<bool> = _serde::__private228::None;
                            let mut __field4: _serde::__private228::Option<String> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("binary"),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "headless",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private228::Option::is_some(&__field2) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "sandbox",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private228::Option::is_some(&__field3) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "disable_gpu",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private228::Option::is_some(&__field4) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "user_data_dir",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("binary")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("headless")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private228::Some(__field2) => __field2,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("sandbox")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private228::Some(__field3) => __field3,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("disable_gpu")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private228::Some(__field4) => __field4,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("user_data_dir")?
                                }
                            };
                            _serde::__private228::Ok(Args {
                                binary: __field0,
                                headless: __field1,
                                sandbox: __field2,
                                disable_gpu: __field3,
                                user_data_dir: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "binary",
                        "headless",
                        "sandbox",
                        "disable_gpu",
                        "user_data_dir",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Args",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<Args>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Neko_Print_Printer_Args_Binary {
            pub target: String,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_Args_Binary {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Args_Binary",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_Args_Binary {
            #[inline]
            fn default() -> Neko_Print_Printer_Args_Binary {
                Neko_Print_Printer_Args_Binary {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Args_Binary {
            pub fn new() -> Self {
                Self {
                    target: <String as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Args_Binary {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "binary",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "binary",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "binary",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "binary",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "binary",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "binary",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "binary",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl Args {
            pub fn print_binary(&self) -> Neko_Print_Printer_Args_Binary {
                Neko_Print_Printer_Args_Binary::new().target(self.binary.clone())
            }
        }
        pub struct Neko_Print_Printer_Args_Headless {
            pub target: bool,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_Args_Headless {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Args_Headless",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_Args_Headless {
            #[inline]
            fn default() -> Neko_Print_Printer_Args_Headless {
                Neko_Print_Printer_Args_Headless {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Args_Headless {
            pub fn new() -> Self {
                Self {
                    target: <bool as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<bool> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Args_Headless {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "headless",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "headless",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "headless",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "headless",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "headless",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "headless",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "headless",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl Args {
            pub fn print_headless(&self) -> Neko_Print_Printer_Args_Headless {
                Neko_Print_Printer_Args_Headless::new().target(self.headless.clone())
            }
        }
        pub struct Neko_Print_Printer_Args_Sandbox {
            pub target: bool,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_Args_Sandbox {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Args_Sandbox",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_Args_Sandbox {
            #[inline]
            fn default() -> Neko_Print_Printer_Args_Sandbox {
                Neko_Print_Printer_Args_Sandbox {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Args_Sandbox {
            pub fn new() -> Self {
                Self {
                    target: <bool as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<bool> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Args_Sandbox {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "sandbox",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "sandbox",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "sandbox",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "sandbox",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "sandbox",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "sandbox",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "sandbox",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl Args {
            pub fn print_sandbox(&self) -> Neko_Print_Printer_Args_Sandbox {
                Neko_Print_Printer_Args_Sandbox::new().target(self.sandbox.clone())
            }
        }
        pub struct Neko_Print_Printer_Args_Disable_gpu {
            pub target: bool,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_Args_Disable_gpu {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Args_Disable_gpu",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_Args_Disable_gpu {
            #[inline]
            fn default() -> Neko_Print_Printer_Args_Disable_gpu {
                Neko_Print_Printer_Args_Disable_gpu {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Args_Disable_gpu {
            pub fn new() -> Self {
                Self {
                    target: <bool as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<bool> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Args_Disable_gpu {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "disable_gpu",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "disable_gpu",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "disable_gpu",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "disable_gpu",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "disable_gpu",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "disable_gpu",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "disable_gpu",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl Args {
            pub fn print_disable_gpu(&self) -> Neko_Print_Printer_Args_Disable_gpu {
                Neko_Print_Printer_Args_Disable_gpu::new()
                    .target(self.disable_gpu.clone())
            }
        }
        pub struct Neko_Print_Printer_Args_User_data_dir {
            pub target: String,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_Args_User_data_dir {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Args_User_data_dir",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_Args_User_data_dir {
            #[inline]
            fn default() -> Neko_Print_Printer_Args_User_data_dir {
                Neko_Print_Printer_Args_User_data_dir {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Args_User_data_dir {
            pub fn new() -> Self {
                Self {
                    target: <String as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Args_User_data_dir {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "user_data_dir",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "user_data_dir",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "user_data_dir",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "user_data_dir",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "user_data_dir",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "user_data_dir",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                "Args",
                                "user_data_dir",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl Args {
            pub fn print_user_data_dir(&self) -> Neko_Print_Printer_Args_User_data_dir {
                Neko_Print_Printer_Args_User_data_dir::new()
                    .target(self.user_data_dir.clone())
            }
        }
        pub struct Neko_Print_Printer_Args {
            pub target: Args,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_Args {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Args",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_Args {
            #[inline]
            fn default() -> Neko_Print_Printer_Args {
                Neko_Print_Printer_Args {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Args {
            pub fn new() -> Self {
                Self {
                    target: <Args as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<Args> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Args {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                7u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
        }
        impl Args {
            pub fn print(&self) -> Neko_Print_Printer_Args {
                Neko_Print_Printer_Args::new().target(self.clone())
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Args {
            #[inline]
            fn default() -> Args {
                Args {
                    binary: ::core::default::Default::default(),
                    headless: ::core::default::Default::default(),
                    sandbox: ::core::default::Default::default(),
                    disable_gpu: ::core::default::Default::default(),
                    user_data_dir: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Args {
            #[inline]
            fn clone(&self) -> Args {
                Args {
                    binary: ::core::clone::Clone::clone(&self.binary),
                    headless: ::core::clone::Clone::clone(&self.headless),
                    sandbox: ::core::clone::Clone::clone(&self.sandbox),
                    disable_gpu: ::core::clone::Clone::clone(&self.disable_gpu),
                    user_data_dir: ::core::clone::Clone::clone(&self.user_data_dir),
                }
            }
        }
        impl Args {
            pub fn to_json(&self) -> Value {
                let mut args = Vec::new();
                if self.headless {
                    args.push("--headless".to_string());
                }
                if !self.sandbox {
                    args.push("--no-sandbox".to_string());
                }
                if self.disable_gpu {
                    args.push("--disable-gpu".to_string());
                }
                if !self.user_data_dir.is_empty() {
                    args.push(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("--user-data-dir={0}", self.user_data_dir),
                            )
                        }),
                    );
                }
                if !self.binary.is_empty() {
                    return ::serde_json::Value::Object({
                        let mut object = ::serde_json::Map::new();
                        let _ = object
                            .insert(
                                ("args").into(),
                                ::serde_json::to_value(&args).unwrap(),
                            );
                        let _ = object
                            .insert(
                                ("binary").into(),
                                ::serde_json::to_value(&self.binary).unwrap(),
                            );
                        object
                    });
                }
                ::serde_json::Value::Object({
                    let mut object = ::serde_json::Map::new();
                    let _ = object
                        .insert(("args").into(), ::serde_json::to_value(&args).unwrap());
                    object
                })
            }
        }
        #[transporter(async fn procedure(){println!("{message}");})]
        pub struct Capabilities {
            pub browser_version: String,
            pub accept_insecure_certs: bool,
            pub page_load_strategy: String,
            pub unhandled_prompt_behavior: String,
            #[set(value = Args::new())]
            pub args: Args,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Capabilities {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Capabilities",
                    "browser_version",
                    &self.browser_version,
                    "accept_insecure_certs",
                    &self.accept_insecure_certs,
                    "page_load_strategy",
                    &self.page_load_strategy,
                    "unhandled_prompt_behavior",
                    &self.unhandled_prompt_behavior,
                    "args",
                    &&self.args,
                )
            }
        }
        impl Capabilities {
            pub fn new() -> Self {
                Self {
                    browser_version: <String as Default>::default(),
                    accept_insecure_certs: <bool as Default>::default(),
                    page_load_strategy: <String as Default>::default(),
                    unhandled_prompt_behavior: <String as Default>::default(),
                    args: Args::new().into(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    browser_version: instance.browser_version,
                    accept_insecure_certs: instance.accept_insecure_certs,
                    page_load_strategy: instance.page_load_strategy,
                    unhandled_prompt_behavior: instance.unhandled_prompt_behavior,
                    args: instance.args,
                }
            }
            pub fn browser_version<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.browser_version = new.into();
                self
            }
            pub fn accept_insecure_certs<New: Into<bool> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.accept_insecure_certs = new.into();
                self
            }
            pub fn page_load_strategy<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.page_load_strategy = new.into();
                self
            }
            pub fn unhandled_prompt_behavior<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.unhandled_prompt_behavior = new.into();
                self
            }
            pub fn args<New: Into<Args> + std::fmt::Debug>(mut self, new: New) -> Self {
                self.args = new.into();
                self
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Capabilities {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Capabilities",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "browser_version",
                        &self.browser_version,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "accept_insecure_certs",
                        &self.accept_insecure_certs,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "page_load_strategy",
                        &self.page_load_strategy,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "unhandled_prompt_behavior",
                        &self.unhandled_prompt_behavior,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "args",
                        &self.args,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Capabilities {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private228::Ok(__Field::__field0),
                                1u64 => _serde::__private228::Ok(__Field::__field1),
                                2u64 => _serde::__private228::Ok(__Field::__field2),
                                3u64 => _serde::__private228::Ok(__Field::__field3),
                                4u64 => _serde::__private228::Ok(__Field::__field4),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "browser_version" => {
                                    _serde::__private228::Ok(__Field::__field0)
                                }
                                "accept_insecure_certs" => {
                                    _serde::__private228::Ok(__Field::__field1)
                                }
                                "page_load_strategy" => {
                                    _serde::__private228::Ok(__Field::__field2)
                                }
                                "unhandled_prompt_behavior" => {
                                    _serde::__private228::Ok(__Field::__field3)
                                }
                                "args" => _serde::__private228::Ok(__Field::__field4),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"browser_version" => {
                                    _serde::__private228::Ok(__Field::__field0)
                                }
                                b"accept_insecure_certs" => {
                                    _serde::__private228::Ok(__Field::__field1)
                                }
                                b"page_load_strategy" => {
                                    _serde::__private228::Ok(__Field::__field2)
                                }
                                b"unhandled_prompt_behavior" => {
                                    _serde::__private228::Ok(__Field::__field3)
                                }
                                b"args" => _serde::__private228::Ok(__Field::__field4),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private228::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private228::PhantomData<Capabilities>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Capabilities;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct Capabilities",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Capabilities with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Capabilities with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Capabilities with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Capabilities with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Args,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Capabilities with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(Capabilities {
                                browser_version: __field0,
                                accept_insecure_certs: __field1,
                                page_load_strategy: __field2,
                                unhandled_prompt_behavior: __field3,
                                args: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                            let mut __field1: _serde::__private228::Option<bool> = _serde::__private228::None;
                            let mut __field2: _serde::__private228::Option<String> = _serde::__private228::None;
                            let mut __field3: _serde::__private228::Option<String> = _serde::__private228::None;
                            let mut __field4: _serde::__private228::Option<Args> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "browser_version",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "accept_insecure_certs",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private228::Option::is_some(&__field2) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "page_load_strategy",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private228::Option::is_some(&__field3) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "unhandled_prompt_behavior",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private228::Option::is_some(&__field4) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("args"),
                                            );
                                        }
                                        __field4 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<Args>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("browser_version")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field(
                                        "accept_insecure_certs",
                                    )?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private228::Some(__field2) => __field2,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field(
                                        "page_load_strategy",
                                    )?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private228::Some(__field3) => __field3,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field(
                                        "unhandled_prompt_behavior",
                                    )?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private228::Some(__field4) => __field4,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("args")?
                                }
                            };
                            _serde::__private228::Ok(Capabilities {
                                browser_version: __field0,
                                accept_insecure_certs: __field1,
                                page_load_strategy: __field2,
                                unhandled_prompt_behavior: __field3,
                                args: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "browser_version",
                        "accept_insecure_certs",
                        "page_load_strategy",
                        "unhandled_prompt_behavior",
                        "args",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Capabilities",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<Capabilities>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Neko_Print_Printer_Capabilities_Browser_version {
            pub target: String,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_Capabilities_Browser_version {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Capabilities_Browser_version",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default
        for Neko_Print_Printer_Capabilities_Browser_version {
            #[inline]
            fn default() -> Neko_Print_Printer_Capabilities_Browser_version {
                Neko_Print_Printer_Capabilities_Browser_version {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Capabilities_Browser_version {
            pub fn new() -> Self {
                Self {
                    target: <String as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Capabilities_Browser_version {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "browser_version",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "browser_version",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "browser_version",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "browser_version",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "browser_version",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "browser_version",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "browser_version",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl Capabilities {
            pub fn print_browser_version(
                &self,
            ) -> Neko_Print_Printer_Capabilities_Browser_version {
                Neko_Print_Printer_Capabilities_Browser_version::new()
                    .target(self.browser_version.clone())
            }
        }
        pub struct Neko_Print_Printer_Capabilities_Accept_insecure_certs {
            pub target: bool,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug
        for Neko_Print_Printer_Capabilities_Accept_insecure_certs {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Capabilities_Accept_insecure_certs",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default
        for Neko_Print_Printer_Capabilities_Accept_insecure_certs {
            #[inline]
            fn default() -> Neko_Print_Printer_Capabilities_Accept_insecure_certs {
                Neko_Print_Printer_Capabilities_Accept_insecure_certs {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Capabilities_Accept_insecure_certs {
            pub fn new() -> Self {
                Self {
                    target: <bool as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<bool> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Capabilities_Accept_insecure_certs {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "accept_insecure_certs",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "accept_insecure_certs",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "accept_insecure_certs",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "accept_insecure_certs",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "accept_insecure_certs",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "accept_insecure_certs",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "accept_insecure_certs",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl Capabilities {
            pub fn print_accept_insecure_certs(
                &self,
            ) -> Neko_Print_Printer_Capabilities_Accept_insecure_certs {
                Neko_Print_Printer_Capabilities_Accept_insecure_certs::new()
                    .target(self.accept_insecure_certs.clone())
            }
        }
        pub struct Neko_Print_Printer_Capabilities_Page_load_strategy {
            pub target: String,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_Capabilities_Page_load_strategy {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Capabilities_Page_load_strategy",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default
        for Neko_Print_Printer_Capabilities_Page_load_strategy {
            #[inline]
            fn default() -> Neko_Print_Printer_Capabilities_Page_load_strategy {
                Neko_Print_Printer_Capabilities_Page_load_strategy {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Capabilities_Page_load_strategy {
            pub fn new() -> Self {
                Self {
                    target: <String as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Capabilities_Page_load_strategy {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "page_load_strategy",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "page_load_strategy",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "page_load_strategy",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "page_load_strategy",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "page_load_strategy",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "page_load_strategy",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "page_load_strategy",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl Capabilities {
            pub fn print_page_load_strategy(
                &self,
            ) -> Neko_Print_Printer_Capabilities_Page_load_strategy {
                Neko_Print_Printer_Capabilities_Page_load_strategy::new()
                    .target(self.page_load_strategy.clone())
            }
        }
        pub struct Neko_Print_Printer_Capabilities_Unhandled_prompt_behavior {
            pub target: String,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug
        for Neko_Print_Printer_Capabilities_Unhandled_prompt_behavior {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Capabilities_Unhandled_prompt_behavior",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default
        for Neko_Print_Printer_Capabilities_Unhandled_prompt_behavior {
            #[inline]
            fn default() -> Neko_Print_Printer_Capabilities_Unhandled_prompt_behavior {
                Neko_Print_Printer_Capabilities_Unhandled_prompt_behavior {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Capabilities_Unhandled_prompt_behavior {
            pub fn new() -> Self {
                Self {
                    target: <String as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Capabilities_Unhandled_prompt_behavior {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "unhandled_prompt_behavior",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "unhandled_prompt_behavior",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "unhandled_prompt_behavior",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "unhandled_prompt_behavior",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "unhandled_prompt_behavior",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "unhandled_prompt_behavior",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "unhandled_prompt_behavior",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl Capabilities {
            pub fn print_unhandled_prompt_behavior(
                &self,
            ) -> Neko_Print_Printer_Capabilities_Unhandled_prompt_behavior {
                Neko_Print_Printer_Capabilities_Unhandled_prompt_behavior::new()
                    .target(self.unhandled_prompt_behavior.clone())
            }
        }
        pub struct Neko_Print_Printer_Capabilities_Args {
            pub target: Args,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_Capabilities_Args {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Capabilities_Args",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_Capabilities_Args {
            #[inline]
            fn default() -> Neko_Print_Printer_Capabilities_Args {
                Neko_Print_Printer_Capabilities_Args {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Capabilities_Args {
            pub fn new() -> Self {
                Self {
                    target: <Args as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<Args> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Capabilities_Args {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "args",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "args",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "args",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "args",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "args",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "args",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                "Capabilities",
                                "args",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl Capabilities {
            pub fn print_args(&self) -> Neko_Print_Printer_Capabilities_Args {
                Neko_Print_Printer_Capabilities_Args::new().target(self.args.clone())
            }
        }
        pub struct Neko_Print_Printer_Capabilities {
            pub target: Capabilities,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_Capabilities {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_Capabilities",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_Capabilities {
            #[inline]
            fn default() -> Neko_Print_Printer_Capabilities {
                Neko_Print_Printer_Capabilities {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_Capabilities {
            pub fn new() -> Self {
                Self {
                    target: <Capabilities as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<Capabilities> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_Capabilities {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                47u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
        }
        impl Capabilities {
            pub fn print(&self) -> Neko_Print_Printer_Capabilities {
                Neko_Print_Printer_Capabilities::new().target(self.clone())
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Capabilities {
            #[inline]
            fn default() -> Capabilities {
                Capabilities {
                    browser_version: ::core::default::Default::default(),
                    accept_insecure_certs: ::core::default::Default::default(),
                    page_load_strategy: ::core::default::Default::default(),
                    unhandled_prompt_behavior: ::core::default::Default::default(),
                    args: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Capabilities {
            #[inline]
            fn clone(&self) -> Capabilities {
                Capabilities {
                    browser_version: ::core::clone::Clone::clone(&self.browser_version),
                    accept_insecure_certs: ::core::clone::Clone::clone(
                        &self.accept_insecure_certs,
                    ),
                    page_load_strategy: ::core::clone::Clone::clone(
                        &self.page_load_strategy,
                    ),
                    unhandled_prompt_behavior: ::core::clone::Clone::clone(
                        &self.unhandled_prompt_behavior,
                    ),
                    args: ::core::clone::Clone::clone(&self.args),
                }
            }
        }
        pub enum Browser {
            #[default]
            Chrome,
            Firefox,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Browser {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Browser::Chrome => "Chrome",
                        Browser::Firefox => "Firefox",
                    },
                )
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Browser {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Browser::Chrome => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "Browser",
                                0u32,
                                "Chrome",
                            )
                        }
                        Browser::Firefox => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "Browser",
                                1u32,
                                "Firefox",
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Browser {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private228::Ok(__Field::__field0),
                                1u64 => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Chrome" => _serde::__private228::Ok(__Field::__field0),
                                "Firefox" => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Chrome" => _serde::__private228::Ok(__Field::__field0),
                                b"Firefox" => _serde::__private228::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private228::from_utf8_lossy(
                                        __value,
                                    );
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private228::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private228::PhantomData<Browser>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Browser;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "enum Browser",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private228::Ok(Browser::Chrome)
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private228::Ok(Browser::Firefox)
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["Chrome", "Firefox"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "Browser",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<Browser>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::default::Default for Browser {
            #[inline]
            fn default() -> Browser {
                Self::Chrome
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Browser {
            #[inline]
            fn clone(&self) -> Browser {
                match self {
                    Browser::Chrome => Browser::Chrome,
                    Browser::Firefox => Browser::Firefox,
                }
            }
        }
        impl Browser {
            pub fn to_json(&self, capabilities: &Capabilities) -> Value {
                match self {
                    Browser::Chrome => {
                        let chrome_options = capabilities.args.to_json();
                        ::serde_json::Value::Object({
                            let mut object = ::serde_json::Map::new();
                            let _ = object
                                .insert(
                                    ("goog:chromeOptions").into(),
                                    ::serde_json::to_value(&chrome_options).unwrap(),
                                );
                            let _ = object
                                .insert(
                                    ("browserName").into(),
                                    ::serde_json::to_value(&"chrome").unwrap(),
                                );
                            let _ = object
                                .insert(
                                    ("platformName").into(),
                                    ::serde_json::to_value(&"linux").unwrap(),
                                );
                            object
                        })
                    }
                    Browser::Firefox => {
                        let firefox_options = capabilities.args.to_json();
                        ::serde_json::Value::Object({
                            let mut object = ::serde_json::Map::new();
                            let _ = object
                                .insert(
                                    ("moz:firefoxOptions").into(),
                                    ::serde_json::to_value(&firefox_options).unwrap(),
                                );
                            let _ = object
                                .insert(
                                    ("browserName").into(),
                                    ::serde_json::to_value(&"firefox").unwrap(),
                                );
                            let _ = object
                                .insert(
                                    ("platformName").into(),
                                    ::serde_json::to_value(&"linux").unwrap(),
                                );
                            object
                        })
                    }
                }
            }
        }
        #[transporter(async fn procedure(){println!("{message}");})]
        pub struct WebDriver {
            #[set(value = "http://localhost:9515")]
            pub url: String,
            pub browser: Browser,
            #[set(value = Capabilities::new())]
            pub capabilities: Capabilities,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for WebDriver {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "WebDriver",
                    "url",
                    &self.url,
                    "browser",
                    &self.browser,
                    "capabilities",
                    &&self.capabilities,
                )
            }
        }
        impl WebDriver {
            pub fn new() -> Self {
                Self {
                    url: "http://localhost:9515".into(),
                    browser: <Browser as Default>::default(),
                    capabilities: Capabilities::new().into(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    url: instance.url,
                    browser: instance.browser,
                    capabilities: instance.capabilities,
                }
            }
            pub fn url<New: Into<String> + std::fmt::Debug>(mut self, new: New) -> Self {
                self.url = new.into();
                self
            }
            pub fn browser<New: Into<Browser> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.browser = new.into();
                self
            }
            pub fn capabilities<New: Into<Capabilities> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.capabilities = new.into();
                self
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for WebDriver {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "WebDriver",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "browser",
                        &self.browser,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "capabilities",
                        &self.capabilities,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for WebDriver {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private228::Ok(__Field::__field0),
                                1u64 => _serde::__private228::Ok(__Field::__field1),
                                2u64 => _serde::__private228::Ok(__Field::__field2),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "url" => _serde::__private228::Ok(__Field::__field0),
                                "browser" => _serde::__private228::Ok(__Field::__field1),
                                "capabilities" => {
                                    _serde::__private228::Ok(__Field::__field2)
                                }
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"url" => _serde::__private228::Ok(__Field::__field0),
                                b"browser" => _serde::__private228::Ok(__Field::__field1),
                                b"capabilities" => {
                                    _serde::__private228::Ok(__Field::__field2)
                                }
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private228::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private228::PhantomData<WebDriver>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = WebDriver;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct WebDriver",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct WebDriver with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Browser,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct WebDriver with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Capabilities,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct WebDriver with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(WebDriver {
                                url: __field0,
                                browser: __field1,
                                capabilities: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                            let mut __field1: _serde::__private228::Option<Browser> = _serde::__private228::None;
                            let mut __field2: _serde::__private228::Option<
                                Capabilities,
                            > = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "browser",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<Browser>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private228::Option::is_some(&__field2) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "capabilities",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Capabilities,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("url")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("browser")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private228::Some(__field2) => __field2,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("capabilities")?
                                }
                            };
                            _serde::__private228::Ok(WebDriver {
                                url: __field0,
                                browser: __field1,
                                capabilities: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "url",
                        "browser",
                        "capabilities",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "WebDriver",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<WebDriver>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Neko_Print_Printer_WebDriver_Url {
            pub target: String,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_WebDriver_Url {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_WebDriver_Url",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_WebDriver_Url {
            #[inline]
            fn default() -> Neko_Print_Printer_WebDriver_Url {
                Neko_Print_Printer_WebDriver_Url {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_WebDriver_Url {
            pub fn new() -> Self {
                Self {
                    target: <String as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_WebDriver_Url {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "url",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "url",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "url",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "url",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "url",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "url",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "url",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl WebDriver {
            pub fn print_url(&self) -> Neko_Print_Printer_WebDriver_Url {
                Neko_Print_Printer_WebDriver_Url::new().target(self.url.clone())
            }
        }
        pub struct Neko_Print_Printer_WebDriver_Browser {
            pub target: Browser,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_WebDriver_Browser {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_WebDriver_Browser",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_WebDriver_Browser {
            #[inline]
            fn default() -> Neko_Print_Printer_WebDriver_Browser {
                Neko_Print_Printer_WebDriver_Browser {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_WebDriver_Browser {
            pub fn new() -> Self {
                Self {
                    target: <Browser as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<Browser> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_WebDriver_Browser {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "browser",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "browser",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "browser",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "browser",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "browser",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "browser",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "browser",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl WebDriver {
            pub fn print_browser(&self) -> Neko_Print_Printer_WebDriver_Browser {
                Neko_Print_Printer_WebDriver_Browser::new().target(self.browser.clone())
            }
        }
        pub struct Neko_Print_Printer_WebDriver_Capabilities {
            pub target: Capabilities,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_WebDriver_Capabilities {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_WebDriver_Capabilities",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_WebDriver_Capabilities {
            #[inline]
            fn default() -> Neko_Print_Printer_WebDriver_Capabilities {
                Neko_Print_Printer_WebDriver_Capabilities {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_WebDriver_Capabilities {
            pub fn new() -> Self {
                Self {
                    target: <Capabilities as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<Capabilities> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_WebDriver_Capabilities {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "capabilities",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "capabilities",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "capabilities",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "capabilities",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "capabilities",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "capabilities",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4}.{5} = {6:#?} {7}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                "WebDriver",
                                "capabilities",
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
            }
        }
        impl WebDriver {
            pub fn print_capabilities(
                &self,
            ) -> Neko_Print_Printer_WebDriver_Capabilities {
                Neko_Print_Printer_WebDriver_Capabilities::new()
                    .target(self.capabilities.clone())
            }
        }
        pub struct Neko_Print_Printer_WebDriver {
            pub target: WebDriver,
            pub message: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Neko_Print_Printer_WebDriver {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Neko_Print_Printer_WebDriver",
                    "target",
                    &self.target,
                    "message",
                    &&self.message,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Neko_Print_Printer_WebDriver {
            #[inline]
            fn default() -> Neko_Print_Printer_WebDriver {
                Neko_Print_Printer_WebDriver {
                    target: ::core::default::Default::default(),
                    message: ::core::default::Default::default(),
                }
            }
        }
        impl Neko_Print_Printer_WebDriver {
            pub fn new() -> Self {
                Self {
                    target: <WebDriver as Default>::default(),
                    message: <String as Default>::default(),
                }
            }
            pub fn build(self) -> Result<Self, String> {
                Ok(self)
            }
            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    target: instance.target,
                    message: instance.message,
                }
            }
            pub fn target<New: Into<WebDriver> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.target = new.into();
                self
            }
            pub fn message<New: Into<String> + std::fmt::Debug>(
                mut self,
                new: New,
            ) -> Self {
                self.message = new.into();
                self
            }
        }
        impl Neko_Print_Printer_WebDriver {
            pub async fn rust(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @RUST => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 165, 0);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn info(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @INFO => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(0, 191, 255);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn success(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @SUCCESS => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .green();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn warning(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @WARNING => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .yellow();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn err(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @ERROR => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(255, 49, 49);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn critical(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @CRITICAL => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .red();
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
            pub async fn panic(&self) {
                use colorful::Colorful;
                let message = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "({0} {1} {2}:{3}) @PANIC => {4:#?} {5}",
                                chrono::Local::now(),
                                "src/components/web_driver.rs",
                                90u32,
                                50u32,
                                &self.target,
                                &self.message,
                            ),
                        )
                    })
                    .rgb(225, 32, 254);
                {
                    ::std::io::_print(format_args!("{0}\n", message));
                };
                (message);
            }
        }
        impl WebDriver {
            pub fn print(&self) -> Neko_Print_Printer_WebDriver {
                Neko_Print_Printer_WebDriver::new().target(self.clone())
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for WebDriver {
            #[inline]
            fn default() -> WebDriver {
                WebDriver {
                    url: ::core::default::Default::default(),
                    browser: ::core::default::Default::default(),
                    capabilities: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for WebDriver {
            #[inline]
            fn clone(&self) -> WebDriver {
                WebDriver {
                    url: ::core::clone::Clone::clone(&self.url),
                    browser: ::core::clone::Clone::clone(&self.browser),
                    capabilities: ::core::clone::Clone::clone(&self.capabilities),
                }
            }
        }
        impl WebDriver {
            pub async fn launch(&self) -> Result<(), Error> {
                let client = Client::builder().build()?;
                let base = self.url.trim_end_matches('/').to_string();
                let endpoint = ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0}/session", base))
                });
                let capabilities_json = self.browser.to_json(&self.capabilities);
                let body = ::serde_json::Value::Object({
                    let mut object = ::serde_json::Map::new();
                    let _ = object
                        .insert(
                            ("capabilities").into(),
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("alwaysMatch").into(),
                                        ::serde_json::to_value(&capabilities_json).unwrap(),
                                    );
                                let _ = object
                                    .insert(
                                        ("firstMatch").into(),
                                        ::serde_json::Value::Array(
                                            <[_]>::into_vec(
                                                ::alloc::boxed::box_new([
                                                    ::serde_json::Value::Object(::serde_json::Map::new()),
                                                ]),
                                            ),
                                        ),
                                    );
                                object
                            }),
                        );
                    object
                });
                {
                    ::std::io::_print(format_args!("{0:#?}\n", body));
                };
                let res = client.post(endpoint).json(&body).send().await?;
                {
                    ::std::io::_print(format_args!("{0:#?}\n", res));
                };
                Ok(())
            }
        }
    }
}
